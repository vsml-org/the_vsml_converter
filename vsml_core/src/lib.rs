use std::{collections::HashMap, sync::Arc};

use schemas::{ObjectProcessor, StyleData};

pub mod schemas;

#[derive(Debug)]
pub struct RenderObject {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
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
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct TextStyleData {
    color: String,
    font_name: String,
}

struct TextData {
    text: String,
    style: TextStyleData,
}

struct Property {
}

struct EffectStyle {
}

struct RenderingInfo {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
struct TextRenderingInfo {
    x: f32,
    y: f32,
    max_width: f32,
    max_height: f32,
}

pub trait Renderer {
    type Image;
    fn render_image(&mut self, image: Self::Image, info: RenderingInfo);
    fn render_text(&mut self, text_data: &[TextData], info: TextRenderingInfo) -> Rect;
    fn render_box(&mut self, property: Property, info: RenderingInfo);
    fn render(self, width: u32, height: u32) -> Self::Image;
}

pub trait ImageStyleApplier {
    type Image;
    fn apply_style(&mut self, image: Self::Image, style: EffectStyle) -> Self::Image;
}

pub trait RendererFactory {
    type Image;
    type Renderer: Renderer<Image=Self::Image>;
    fn create_renderer(&mut self) -> Self::Renderer;
}

fn convert_to_render_info<R>(
    iv_data: &schemas::IVData<<R::Renderer as Renderer>::Image>,
    frame_number: usize,
    mut rendering_context: R,
) -> <R::Renderer as Renderer>::Image
where
    R: RendererFactory<Image=<R as ImageStyleApplier>::Image> + ImageStyleApplier,
{
    // let mut renderer0 = rendering_context.create_renderer();
    // let text_rect =renderer0.render_text();
    //     let mut renderer = rendering_context.create_renderer();
    //     render.render_text();
    //     renderer.render_image();
    //     let child_image = renderer.render(200, 200);
    //     let image = object_processor.process(child_image, attributes);
    // renderer0.render_image(image);
    todo!()
}

fn convert_to_mix_info(iv_data: &schemas::IVData) -> MixData {
    todo!()
}
