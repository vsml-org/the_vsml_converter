use std::collections::HashMap;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::ObjectProcessor;

pub struct ImageProcessor;

impl ObjectProcessor<VsmlImage> for ImageProcessor {
    fn name(&self) -> &str {
        "image"
    }

    fn default_duration(&self, _attributes: HashMap<String, String>) -> f64 {
        f64::INFINITY
    }

    fn process(
        &self,
        _: f64,
        attributes: &HashMap<String, String>,
        _: Option<VsmlImage>,
    ) -> VsmlImage {
        let src_path = attributes.get("src").unwrap();
        image::open(src_path).unwrap().into_rgba8()
    }
}
