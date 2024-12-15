use std::collections::HashMap;
use std::sync::Arc;
use vsml_core::{render_frame_image, ElementRect};
use vsml_core::Alignment::Center;
use vsml_core::schemas::{IVData, ObjectData, ObjectType, StyleData};
use vsml_processer::ImageProcessor;
use vsml_renderer::RenderingContextImpl;

fn main() {
    let iv_data = IVData {
        resolution_x: 1920,
        resolution_y: 1080,
        fps: 30,
        sampling_rate: 44100,
        object: ObjectData::Element {
            object_type: ObjectType::Wrap,
            start_time: 0.0,
            duration: 1.0,
            attributes: HashMap::new(),
            element_rect: ElementRect {
                alignment: Default::default(),
                parent_alignment: Default::default(),
                x: 0.0,
                y: 0.0,
                width: 1920.0,
                height: 1080.0,
            },
            styles: StyleData::default(),
            children: vec![
                ObjectData::Element {
                    object_type: ObjectType::Other(Arc::new(ImageProcessor)),
                    start_time: 0.0,
                    duration: 1.0,
                    attributes: HashMap::from([("src".to_owned(), "image.png".to_owned())]),
                    element_rect: ElementRect {
                        alignment: Center,
                        parent_alignment: Center,
                        x: 0.0,
                        y: 0.0,
                        width: 350.0,
                        height: 350.0,
                    },
                    styles: StyleData::default(),
                    children: vec![],
                },
            ],
        }
    };

    let mut rendering_context = RenderingContextImpl::new();

    for f in 0..10 {
        let frame_image = render_frame_image(&iv_data, f, &mut rendering_context);
        frame_image.save(format!("output/frame_{:04}.png", f)).unwrap();
    }
}
