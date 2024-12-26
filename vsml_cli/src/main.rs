use std::collections::HashMap;
use std::sync::Arc;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::ObjectProcessor;
use vsml_encoder::encode;
use vsml_iv_converter::convert;
use vsml_parser::{parse, VSSLoader};
use vsml_processer::ImageProcessor;
use vsml_renderer::RenderingContextImpl;

struct VSSFileLoader;

impl VSSLoader for VSSFileLoader {
    type Err = std::io::Error;
    fn load(&self, path: &str) -> Result<String, Self::Err> {
        std::fs::read_to_string(path)
    }
}

fn main() {
    let vsml_file_path = std::env::args().nth(1).unwrap_or("video.vsml".to_string());
    let vsml_string = std::fs::read_to_string(&vsml_file_path).unwrap();
    let vsml = parse(&vsml_string, &VSSFileLoader).unwrap();
    let iv_data = convert(
        &vsml,
        &HashMap::from([(
            "img".to_string(),
            Arc::new(ImageProcessor) as Arc<dyn ObjectProcessor<VsmlImage>>,
        )]),
    );

    let mut rendering_context = RenderingContextImpl::new();

    encode(iv_data, &mut rendering_context);
}
