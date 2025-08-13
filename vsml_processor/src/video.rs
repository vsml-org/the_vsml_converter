use image::{RgbaImage, load_from_memory};
use std::collections::HashMap;
use std::process::Command;
use vsml_common_audio::Audio as VsmlAudio;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::{ObjectProcessor, RectSize};

pub struct VideoProcessor {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl VideoProcessor {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self { device, queue }
    }

    fn get_frame(&self, src_path: &str, target_time: f64) -> Result<RgbaImage, ()> {
        // ffmpegのコマンドを構築して、指定された時間のフレームの画像を取得
        let output = Command::new("ffmpeg")
            .arg("-ss")
            .arg(target_time.to_string())
            .arg("-i")
            .arg(src_path)
            .arg("-frames:v")
            .arg("1")
            .arg("-f")
            .arg("image2pipe")
            .arg("-vcodec")
            .arg("png")
            .arg("-")
            .output()
            .unwrap()
            .stdout;

        // PNG画像をデコードして、幅と高さを取得
        let img = match load_from_memory(&output) {
            Ok(img) => img.to_rgba8(),
            Err(_) => {
                println!("output: {:?}", output);
                println!("target_time: {}", target_time);
                panic!("failed to decode image from video frame");
            }
        };
        Ok(img)
    }

    fn get_last_pts_time(&self, src_path: &str) -> Result<f64, ()> {
        let output = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-select_streams")
            .arg("v:0")
            .arg("-show_entries")
            .arg("frame=pts_time")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(src_path)
            .output()
            .unwrap()
            .stdout;

        let pts_times = String::from_utf8_lossy(&output);
        let last_pts_time = pts_times.trim().lines().last().unwrap().parse().unwrap();
        Ok(last_pts_time)
    }
}

impl ObjectProcessor<VsmlImage, VsmlAudio> for VideoProcessor {
    fn name(&self) -> &str {
        "video"
    }

    fn default_duration(&self, attributes: &HashMap<String, String>) -> f64 {
        let src_path = attributes.get("src").unwrap();

        let output = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-show_entries")
            .arg("format=duration")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(src_path)
            .output()
            .unwrap();
        let timestamps = String::from_utf8_lossy(&output.stdout);
        timestamps.lines().last().unwrap().trim().parse().unwrap()
    }

    fn default_image_size(&self, attributes: &HashMap<String, String>) -> RectSize {
        let src_path = attributes.get("src").unwrap();
        let output = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-select_streams")
            .arg("v:0")
            .arg("-show_entries")
            .arg("stream=width,height")
            .arg("-of")
            .arg("csv=p=0")
            .arg(src_path)
            .output()
            .unwrap()
            .stdout;

        let rect = String::from_utf8_lossy(&output);
        let rect: Vec<&str> = rect.trim().split(",").collect();
        RectSize::new(rect[0].parse().unwrap(), rect[1].parse().unwrap())
    }

    fn process_image(
        &self,
        target_time: f64,
        attributes: &HashMap<String, String>,
        _: Option<VsmlImage>,
    ) -> Option<VsmlImage> {
        let src_path = attributes.get("src").unwrap();

        let last_pts_time = self.get_last_pts_time(src_path).unwrap();
        let target_time = target_time.min(last_pts_time);
        let frame = self.get_frame(src_path, target_time).unwrap();

        let (width, height) = frame.dimensions();
        let data = &frame.into_raw();

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );
        Some(texture)
    }

    fn process_audio(
        &self,
        attributes: &HashMap<String, String>,
        _audio: Option<VsmlAudio>,
    ) -> Option<VsmlAudio> {
        let src_path = attributes.get("src").unwrap();

        // サンプリングレートの取得
        let rate_output = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-select_streams")
            .arg("a:0")
            .arg("-show_entries")
            .arg("stream=sample_rate")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(src_path)
            .output()
            .unwrap();
        let sampling_rate: u32 = String::from_utf8_lossy(&rate_output.stdout)
            .trim()
            .parse()
            .unwrap();

        let raw_data = Command::new("ffmpeg")
            .arg("-i")
            .arg(src_path)
            .arg("-f")
            .arg("f32le")
            .arg("-ac")
            .arg("2")
            .arg("-acodec")
            .arg("pcm_f32le")
            .arg("-")
            .output()
            .unwrap()
            .stdout;

        let (floats, _) = raw_data.as_chunks::<4>();
        let mut samples = Vec::with_capacity(floats.len() / 2);
        for chunk in floats.chunks_exact(2) {
            let left = f32::from_ne_bytes(chunk[0]);
            let right = f32::from_ne_bytes(chunk[1]);
            samples.push([left, right]);
        }

        Some(VsmlAudio {
            samples,
            sampling_rate,
        })
    }
}
