use image::load_from_memory;
use mp4parse::{read_mp4, SampleEntry};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Cursor, Read};
use std::process::{Command, Stdio};
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

    fn get_frame(&self, src_path: &str, target_time: f64) -> Result<(Vec<u8>, u32, u32), ()> {
        // ffmpegのコマンドを構築して、指定された時間のフレームの画像を取得
        let mut command = Command::new("ffmpeg")
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
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let mut output = vec![];
        if let Some(mut stdout) = command.stdout.take() {
            stdout.read_to_end(&mut output).unwrap();
        }
        command.wait().unwrap();

        // PNG画像をデコードして、幅と高さを取得
        let img = match load_from_memory(&output) {
            Ok(img) => img.to_rgba8(),
            Err(_) => {
                println!("output: {:?}", output);
                println!("target_time: {}", target_time);
                panic!("failed to decode image from video frame");
            }
        };
        let (width, height) = img.dimensions();
        let data = img.into_raw();

        Ok((data, width, height))
    }

    fn get_pts_frame(&self, src_path: &str) -> Result<f64, ()> {
        let mut output = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-select_streams")
            .arg("v:0")
            .arg("-show_entries")
            .arg("frame=pts_time")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(src_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let stdout = output.stdout.take().unwrap();
        output.wait().unwrap();

        let reader = std::io::BufReader::new(stdout);

        let pts_frame = reader.lines().last().unwrap().unwrap();
        Ok(pts_frame.parse().unwrap())
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
            .ok()
            .unwrap();
        let timestamps = String::from_utf8_lossy(&output.stdout);
        timestamps
            .lines()
            .last()
            .unwrap()
            .trim()
            .parse::<f64>()
            .ok()
            .unwrap()
    }

    fn default_image_size(&self, attributes: &HashMap<String, String>) -> RectSize {
        let src_path = attributes.get("src").unwrap();
        let mut file = File::open(src_path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let mut reader = Cursor::new(&buffer);
        match read_mp4(&mut reader) {
            Ok(context) => {
                for track in &context.tracks {
                    if let Some(ref stsd) = track.stsd {
                        for sample in &stsd.descriptions {
                            if let SampleEntry::Video(video) = sample {
                                return RectSize::new(video.width as f32, video.height as f32);
                            }
                        }
                    }
                }
                panic!("Error reading mp4: {:?}", src_path);
            }
            Err(_) => {
                panic!("Error reading mp4: {:?}", src_path);
            }
        }
    }

    fn process_image(
        &self,
        target_time: f64,
        attributes: &HashMap<String, String>,
        _: Option<VsmlImage>,
    ) -> Option<VsmlImage> {
        let src_path = attributes.get("src").unwrap();

        let target_time = target_time.min(self.get_pts_frame(src_path).unwrap());
        let frame = self.get_frame(src_path, target_time).unwrap();

        let size = wgpu::Extent3d {
            width: frame.1,
            height: frame.2,
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
            &frame.0,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * frame.1),
                rows_per_image: Some(frame.2),
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
            .stderr(Stdio::null())
            .output()
            .unwrap();
        let sampling_rate: u32 = String::from_utf8_lossy(&rate_output.stdout)
            .trim()
            .parse()
            .unwrap_or(44100);
        // ffmpegでPCM(f32, stereo)を出力
        let mut child = Command::new("ffmpeg")
            .arg("-i")
            .arg(src_path)
            .arg("-f")
            .arg("f32le") // raw PCM float32
            .arg("-ac")
            .arg("2") // stereo
            .arg("-acodec")
            .arg("pcm_f32le")
            .arg("-") // stdout
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let mut raw_data = Vec::new();
        if let Some(mut stdout) = child.stdout.take() {
            stdout.read_to_end(&mut raw_data).unwrap();
        }
        child.wait().unwrap();

        // バイナリ → f32 に変換
        let floats: &[f32] = unsafe {
            std::slice::from_raw_parts(raw_data.as_ptr() as *const f32, raw_data.len() / 4)
        };

        // L,R でまとめる
        let mut samples = Vec::with_capacity(floats.len() / 2);
        for chunk in floats.chunks_exact(2) {
            samples.push([chunk[0], chunk[1]]);
        }

        Some(VsmlAudio {
            samples,
            sampling_rate,
        })
    }
}
