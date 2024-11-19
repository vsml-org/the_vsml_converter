use std::{collections::HashMap, sync::Arc};

use schemas::{ObjectProcessor, StyleData};

pub mod schemas;

#[derive(Debug)]
pub struct RenderObject {
    width: f64,
    height: f64,
    x: f64,
    y: f64,
    src: Arc<dyn ObjectProcessor>,
    attributes: HashMap<String, String>,
    children: Vec<RenderObject>,
    style: StyleData,
}

#[derive(Debug)]
pub struct RenderData {
    frame: usize,
    objects: Vec<RenderObject>,
}

#[derive(Debug)]
pub struct MixData {}

pub struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

struct TextStyleData {}

struct TextData {
    text: String,
    style: TextStyleData,
}

pub trait Renderer {
    type Image;
    fn render_image(&mut self, x, y, width, height, Self::Image);
    fn render_text(&mut self, x, y, max_width, max_height, text_data: &[TextData]) -> Rect;
    fn render_box(&mut self, x, y, width, height, (/* Style */));
    fn render(self, width: u32, height: u32) -> Self::Image;
}

pub trait RendererCreator {
    type Renderer: Renderer;
    fn create_renderer(&mut self) -> Self::Renderer;
    fn apply_style(&mut self, <Self::Renderer as Renderer>::Image, (/* Style */)) -> <Self::Renderer as Renderer>::Image;
}

fn convert_to_render_info<R>(
    iv_data: &schemas::IVData<<R::Renderer as Renderer>::Image>,
    frame_number: usize,
    mut renderer_creator: R,
) -> <R::Renderer as Renderer>::Image
where
    R: RendererCreator,
{
    let mut renderer0 = renderer_creator.create_renderer();
    let text_rect =renderer0.render_text();
        let mut renderer = renderer_creator.create_renderer();
        // render.render_text();
        renderer.render_image();
        let child_image = renderer.render(200, 200);
        let image = object_processor.process(child_image, attributes);
    renderer0.render_image(image);
    todo!()
}

fn convert_to_mix_info(iv_data: &schemas::IVData) -> MixData {
    todo!()
}
