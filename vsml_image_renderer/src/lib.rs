use vsml_common_image::Image as VsmlImage;
use vsml_core::{Rect, RenderingInfo};

pub struct RendererImpl {
    images: Vec<(VsmlImage, RenderingInfo)>,
}

pub struct RenderingContextImpl {}

impl vsml_core::Renderer for RendererImpl {
    type Image = VsmlImage;

    fn render_image(&mut self, image: Self::Image, info: vsml_core::RenderingInfo) {
        self.images.push((image, info));
    }

    fn render_text(
        &mut self,
        _text_data: &[vsml_core::TextData],
        _info: vsml_core::TextRenderingInfo,
    ) -> Rect {
        todo!()
    }

    fn render_box(&mut self, _property: vsml_core::Property, _info: RenderingInfo) {
        todo!()
    }

    fn render(self, width: u32, height: u32) -> Self::Image {
        let mut result = VsmlImage::new(width, height);
        self.images.iter().for_each(|(image, info)| {
            image::imageops::resize(
                image,
                info.width as u32,
                info.height as u32,
                image::imageops::FilterType::CatmullRom,
            );
            image::imageops::overlay(&mut result, image, info.x as i64, info.y as i64);
        });
        result
    }
}

impl Default for RenderingContextImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderingContextImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl vsml_core::RenderingContext for RenderingContextImpl {
    type Image = VsmlImage;
    type Renderer = RendererImpl;

    fn create_renderer(&mut self) -> Self::Renderer {
        RendererImpl { images: vec![] }
    }

    fn apply_style(&mut self, image: Self::Image, _style: vsml_core::ImageEffectStyle) -> Self::Image {
        image
    }
}
