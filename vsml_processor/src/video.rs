use ffmpeg_next as ffmpeg;
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

    fn get_frame(&self, src_path: &str, target_time: f64) -> Result<ffmpeg::frame::Video, ()> {
        ffmpeg::init().unwrap();

        let mut ictx = ffmpeg::format::input(&src_path).unwrap();
        let input = ictx.streams().best(ffmpeg::media::Type::Video).unwrap();
        let video_stream_index = input.index();
        let time_base = input.time_base();

        let decoder = input.codec().decoder().video().unwrap();
        let mut scaler = ffmpeg::software::scaling::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            ffmpeg::format::Pixel::RGBA,
            decoder.width(),
            decoder.height(),
            ffmpeg::software::scaling::Flags::BILINEAR,
        )
        .unwrap();

        let mut decoder = decoder;
        let mut received_frame = ffmpeg::frame::Video::empty();
        let mut rgba_frame = ffmpeg::frame::Video::empty();

        let target_pts = target_time / time_base.into();

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                if let Some(packet_pts) = packet.pts() {
                    if packet_pts >= target_pts {
                        decoder.send_packet(&packet).unwrap();
                        while decoder.receive_frame(&mut received_frame).is_ok() {
                            if let Some(frame_pts) = received_frame.pts() {
                                if frame_pts >= target_pts {
                                    scaler.run(&received_frame, &mut rgba_frame)?;
                                    return Ok(rgba_frame);
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(())
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

        let frame = self.get_frame(src_path, target_time).unwrap();

        let data = frame.data(0);
        let width = frame.width();
        let height = frame.height();

        let dimensions = (width, height);
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
            data,
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
        // TODO: Implement audio processing for video
        None
    }
}
