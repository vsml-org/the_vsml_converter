use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use mp4parse::{read_mp4, SampleEntry};
use vsml_common_image::Image as VsmlImage;
use vsml_common_audio::Audio as VsmlAudio;
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

impl<A> ObjectProcessor<VsmlImage, VsmlAudio> for VideoProcessor {
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
            Err(e) => {
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
            Err(e) => {
                panic!("Error reading mp4: {:?}", src_path);
            }
        }
    }

    fn process_image(
        &self,
        _: f64,
        attributes: &HashMap<String, String>,
        _: Option<VsmlImage>,
    ) -> Option<VsmlImage> {
        todo!();
    }

    fn process_audio(&self, _attributes: &HashMap<String, String>, _audio: Option<A>) -> Option<A> {
        todo!();
    }
}
