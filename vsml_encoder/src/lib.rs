use std::collections::HashSet;
use std::path::Path;
use std::process::Command;
use temp_dir::TempDir;
use vsml_common_audio::Audio as VsmlAudio;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::{IVData, ObjectData};
use vsml_core::{MixingContext, RenderingContext, mix_audio, render_frame_image};
use wgpu::util::DeviceExt;

/// フレームごとのアクティブな要素を計算
fn calculate_frame_changes<I, A>(iv_data: &IVData<I, A>) -> Vec<HashSet<String>> {
    let ObjectData::Element { duration, .. } = &iv_data.object else {
        panic!()
    };
    let whole_frames = (*duration * iv_data.fps as f64).round() as u32;
    let mut frame_elements: Vec<HashSet<String>> = vec![HashSet::new(); whole_frames as usize];

    fn collect_active_elements<I, A>(
        object: &ObjectData<I, A>,
        frame_elements: &mut [HashSet<String>],
        fps: u32,
        path: String,
        parent_start: f64,
    ) {
        match object {
            ObjectData::Element {
                start_time,
                duration,
                children,
                attributes: _,
                ..
            } => {
                let global_start = parent_start + start_time;
                let start_frame = (global_start * fps as f64).floor() as u32;
                let end_frame = ((global_start + duration) * fps as f64).ceil() as u32;

                // この要素がアクティブなフレームを記録
                for frame in start_frame..end_frame.min(frame_elements.len() as u32) {
                    frame_elements[frame as usize].insert(path.clone());
                }

                // 子要素を再帰的に処理
                for (i, child) in children.iter().enumerate() {
                    collect_active_elements(
                        child,
                        frame_elements,
                        fps,
                        format!("{}/{}", path, i),
                        global_start,
                    );
                }
            }
            ObjectData::Text(_) => {}
        }
    }

    collect_active_elements(
        &iv_data.object,
        &mut frame_elements,
        iv_data.fps,
        "root".to_string(),
        0.0,
    );
    frame_elements
}

pub fn encode<R, M>(
    iv_data: IVData<R::Image, M::Audio>,
    mut rendering_context: R,
    mut mixing_context: M,
    output_path: Option<&Path>,
    overwrite: bool,
    device: wgpu::Device,
    queue: wgpu::Queue,
) where
    R: RenderingContext<Image = VsmlImage>,
    M: MixingContext<Audio = VsmlAudio>,
{
    let ObjectData::Element { duration, .. } = iv_data.object else {
        panic!()
    };
    assert_ne!(duration, 0.0, "動画時間が0秒になっています");
    assert!(duration.is_finite(), "動画時間が無限になっています");
    let whole_frames = duration * iv_data.fps as f64;

    let d = TempDir::new().unwrap();
    let d = d.path();

    // フレームごとのアクティブな要素を事前計算
    let frame_changes = calculate_frame_changes(&iv_data);

    // 前フレームの情報を保持
    let mut last_frame_elements: Option<HashSet<String>> = None;
    let mut last_frame_path: Option<String> = None;

    for f in 0..whole_frames.round() as u32 {
        let save_path = d.join(format!("frame_{}.png", f));

        let current_elements = &frame_changes[f as usize];

        // フレーム間の変化をチェック
        let should_reuse = last_frame_elements
            .as_ref()
            .map(|last| last == current_elements)
            .unwrap_or(false);

        if should_reuse && last_frame_path.is_some() {
            // 前フレームをコピー
            let last_path = last_frame_path.as_ref().unwrap();
            std::fs::copy(last_path, &save_path).unwrap();
        } else {
            // 新規レンダリング
            let frame_image = render_frame_image(&iv_data, f, &mut rendering_context);

            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &vec![
                    0u8;
                    iv_data.resolution_x as usize * iv_data.resolution_y as usize * 4
                ],
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            });
            encoder.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    texture: &frame_image,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * iv_data.resolution_x),
                        rows_per_image: Some(iv_data.resolution_y),
                    },
                },
                wgpu::Extent3d {
                    width: iv_data.resolution_x,
                    height: iv_data.resolution_y,
                    depth_or_array_layers: 1,
                },
            );
            queue.submit(std::iter::once(encoder.finish()));

            let slice = &buffer.slice(..);
            slice.map_async(wgpu::MapMode::Read, |_| {});

            device.poll(wgpu::MaintainBase::Wait);

            image::save_buffer(
                &save_path,
                &slice.get_mapped_range(),
                iv_data.resolution_x,
                iv_data.resolution_y,
                image::ColorType::Rgba8,
            )
            .unwrap();

            // 現在のフレーム情報を保存
            last_frame_elements = Some(current_elements.clone());
            last_frame_path = Some(save_path.to_string_lossy().to_string());
        }
    }

    let audio = mix_audio(&iv_data, &mut mixing_context);

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: audio.sampling_rate,
        sample_format: hound::SampleFormat::Float,
        bits_per_sample: 32,
    };
    let mut writer = hound::WavWriter::create(d.join("audio.wav"), spec).unwrap();
    audio.samples.iter().for_each(|s| {
        writer.write_sample(s[0]).unwrap();
        writer.write_sample(s[1]).unwrap();
    });
    writer.finalize().unwrap();

    let fps = iv_data.fps.to_string();
    let output_path = output_path.unwrap_or(Path::new("output.mp4"));

    let mut command = Command::new("ffmpeg");
    if overwrite {
        command.arg("-y");
    }
    command
        .arg("-r")
        .arg(&fps)
        .arg("-i")
        .arg(d.join("frame_%d.png"))
        .arg("-i")
        .arg(d.join("audio.wav"))
        .arg("-vcodec")
        .arg("libx264")
        .arg("-acodec")
        .arg("aac")
        .arg(output_path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
