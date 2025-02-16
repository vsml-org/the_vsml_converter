use std::path::Path;
use std::process::Command;
use temp_dir::TempDir;
use vsml_common_audio::Audio as VsmlAudio;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::{IVData, ObjectData};
use vsml_core::{mix_audio, render_frame_image, MixingContext, RenderingContext};
use wgpu::util::DeviceExt;

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

    for f in 0..whole_frames.round() as u32 {
        let frame_image = render_frame_image(&iv_data, f, &mut rendering_context);
        let save_path = d.join(format!("frame_{}.png", f));

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &vec![0u8; iv_data.resolution_x as usize * iv_data.resolution_y as usize * 4],
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
            save_path,
            &slice.get_mapped_range().to_vec(),
            iv_data.resolution_x,
            iv_data.resolution_y,
            image::ColorType::Rgba8,
        )
        .unwrap();
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
