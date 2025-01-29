use vsml_core::{ImageEffectStyle, Property, Rect, Renderer, RenderingContext, RenderingInfo, TextData, TextRenderingInfo};
use vsml_common_image::Image as VsmlImage;

pub struct RendererImpl {
    images: Vec<(VsmlImage, RenderingInfo)>,
}

pub struct RenderingContextImpl {}

impl Renderer for RendererImpl {
    type Image = VsmlImage;

    fn render_image(&mut self, image: Self::Image, info: RenderingInfo) {
        todo!()
    }

    fn render_text(&mut self, text_data: &[TextData], info: TextRenderingInfo) -> Rect {
        todo!()
    }

    fn render_box(&mut self, property: Property, info: RenderingInfo) {
        todo!()
    }

    fn render(self, width: u32, height: u32) -> Self::Image {
        todo!()
    }
}

impl RenderingContext for RenderingContextImpl {
    type Image = VsmlImage;
    type Renderer = RendererImpl;

    fn create_renderer(&mut self) -> Self::Renderer {
        todo!()
    }

    fn apply_style(&mut self, image: Self::Image, style: ImageEffectStyle) -> Self::Image {
        todo!()
    }
}
