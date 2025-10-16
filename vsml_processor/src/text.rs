use vsml_common_audio::Audio as VsmlAudio;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::{ObjectProcessor, RectSize};

pub struct TextProcessor {
    _device: wgpu::Device,
    _queue: wgpu::Queue,
}

impl TextProcessor {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self {
            _device: device,
            _queue: queue,
        }
    }
}

impl ObjectProcessor<VsmlImage, VsmlAudio> for TextProcessor {
    fn name(&self) -> &str {
        "text"
    }

    fn default_duration(&self, _: &std::collections::HashMap<String, String>) -> f64 {
        f64::INFINITY
    }

    fn default_image_size(
        &self,
        _: &std::collections::HashMap<String, String>,
    ) -> vsml_core::schemas::RectSize {
        RectSize::ZERO
    }

    fn process_image(
        &self,
        _time: f64,
        _attributes: &std::collections::HashMap<String, String>,
        _input: Option<VsmlImage>,
    ) -> Option<VsmlImage> {
        None
    }

    fn process_audio(
        &self,
        _: &std::collections::HashMap<String, String>,
        _: Option<VsmlAudio>,
    ) -> Option<VsmlAudio> {
        None
    }
}
