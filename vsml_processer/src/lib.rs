use std::collections::HashMap;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::ObjectProcessor;

pub struct ImageProcessor;

impl<A> ObjectProcessor<VsmlImage, A> for ImageProcessor {
    fn name(&self) -> &str {
        "image"
    }

    fn default_duration(&self, _attributes: &HashMap<String, String>) -> f64 {
        f64::INFINITY
    }

    fn process_image(
        &self,
        _: f64,
        attributes: &HashMap<String, String>,
        _: Option<VsmlImage>,
    ) -> Option<VsmlImage> {
        let src_path = attributes.get("src").unwrap();
        Some(image::open(src_path).unwrap().into_rgba8())
    }

    fn process_audio(&self, attributes: &HashMap<String, String>, audio: Option<A>) -> Option<A> {
        None
    }
}
