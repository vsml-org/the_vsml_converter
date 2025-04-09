use image::GenericImageView;
use mp4parse::{read_mp4, SampleEntry};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
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
}

impl ObjectProcessor<VsmlImage, VsmlAudio> for VideoProcessor {
    fn name(&self) -> &str {
        "video"
    }

    fn default_duration(&self, attributes: &HashMap<String, String>) -> f64 {
        let src_path = attributes.get("src").unwrap();
        let mut file = File::open(src_path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let mut reader = Cursor::new(&buffer);
        match read_mp4(&mut reader) {
            Ok(context) => {
                for track in &context.tracks {
                    if let (Some(timescale), Some(duration)) = (track.timescale, track.duration) {
                        return duration.0 as f64 / timescale.0 as f64;
                    }
                }
                panic!("Error reading mp4: {:?}", src_path);
            }
            Err(_) => {
                panic!("Error reading mp4: {:?}", src_path);
            }
        }
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

        // todo: ここで動画から1フレームを取得する

        let dimensions = image.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
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
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );
        Some(texture)
    }

    fn process_audio(
        &self,
        _attributes: &HashMap<String, String>,
        _audio: Option<VsmlAudio>,
    ) -> Option<VsmlAudio> {
        todo!();
    }
}
