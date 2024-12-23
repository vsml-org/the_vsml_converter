use crate::schemas::{ObjectData, ObjectType};

pub mod schemas;
#[cfg(test)]
mod tests;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct TextStyleData {
    pub color: String,
    pub font_name: String,
}

pub struct TextData {
    pub text: String,
    pub style: TextStyleData,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Alignment {
    Center,
    Top,
    Left,
    Right,
    Bottom,
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

enum AlignmentSingle {
    Start,
    Center,
    End,
}

impl Alignment {
    fn x_axis(&self) -> AlignmentSingle {
        match self {
            Alignment::TopLeft | Alignment::Left | Alignment::BottomLeft => AlignmentSingle::Start,
            Alignment::Top | Alignment::Center | Alignment::Bottom => AlignmentSingle::Center,
            Alignment::TopRight | Alignment::Right | Alignment::BottomRight => AlignmentSingle::End,
        }
    }
    fn y_axis(&self) -> AlignmentSingle {
        match self {
            Alignment::TopLeft | Alignment::Top | Alignment::TopRight => AlignmentSingle::Start,
            Alignment::Left | Alignment::Center | Alignment::Right => AlignmentSingle::Center,
            Alignment::BottomLeft | Alignment::Bottom | Alignment::BottomRight => {
                AlignmentSingle::End
            }
        }
    }
}

/// Alignment付きのRectの位置とサイズ
#[derive(Debug)]
pub struct ElementRect {
    pub alignment: Alignment,
    pub parent_alignment: Alignment,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl ElementRect {
    fn calc_rendering_info(&self, outer_width: f32, outer_height: f32) -> RenderingInfo {
        let x = self.x
            - match self.alignment.x_axis() {
                AlignmentSingle::Start => 0.0,
                AlignmentSingle::Center => self.width / 2.0,
                AlignmentSingle::End => self.width,
            }
            + match self.parent_alignment.x_axis() {
                AlignmentSingle::Start => 0.0,
                AlignmentSingle::Center => outer_width / 2.0,
                AlignmentSingle::End => outer_width,
            };
        let y = self.y
            - match self.alignment.y_axis() {
                AlignmentSingle::Start => 0.0,
                AlignmentSingle::Center => self.height / 2.0,
                AlignmentSingle::End => self.height,
            }
            + match self.parent_alignment.y_axis() {
                AlignmentSingle::Start => 0.0,
                AlignmentSingle::Center => outer_height / 2.0,
                AlignmentSingle::End => outer_height,
            };
        RenderingInfo {
            x,
            y,
            width: self.width,
            height: self.height,
        }
    }
}

pub struct Property {}

pub struct EffectStyle {}

/// rendererから見た左上の座標とサイズ
#[derive(Debug)]
pub struct RenderingInfo {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
pub struct TextRenderingInfo {
    pub x: f32,
    pub y: f32,
    pub max_width: f32,
    pub max_height: f32,
}

#[cfg_attr(test, mockall::automock(type Image=tests::MockImage;))]
pub trait Renderer {
    type Image;
    fn render_image(&mut self, image: Self::Image, info: RenderingInfo);
    fn render_text(&mut self, text_data: &[TextData], info: TextRenderingInfo) -> Rect;
    fn render_box(&mut self, property: Property, info: RenderingInfo);
    fn render(self, width: u32, height: u32) -> Self::Image;
}

#[cfg_attr(test, mockall::automock(type Image=tests::MockImage; type Renderer=MockRenderer;))]
pub trait RenderingContext {
    type Image;
    type Renderer: Renderer<Image = Self::Image>;
    fn create_renderer(&mut self) -> Self::Renderer;
    fn apply_style(&mut self, image: Self::Image, style: EffectStyle) -> Self::Image;
}

impl<R> RenderingContext for &mut R
where
    R: RenderingContext,
{
    type Image = R::Image;
    type Renderer = R::Renderer;
    fn create_renderer(&mut self) -> Self::Renderer {
        R::create_renderer(self)
    }
    fn apply_style(&mut self, image: Self::Image, style: EffectStyle) -> Self::Image {
        R::apply_style(self, image, style)
    }
}

pub fn render_frame_image<R>(
    &schemas::IVData {
        resolution_x,
        resolution_y,
        fps,
        sampling_rate: _,
        ref object,
    }: &schemas::IVData<R::Image>,
    frame_number: u32,
    mut rendering_context: R,
) -> R::Image
where
    R: RenderingContext,
{
    fn render_inner<R>(
        rendering_context: &mut R,
        renderer: &mut R::Renderer,
        object: &ObjectData<R::Image>,
        target_time: f64,
        outer_width: f32,
        outer_height: f32,
    ) where
        R: RenderingContext,
    {
        match object {
            &ObjectData::Element {
                ref object_type,
                start_time,
                duration,
                ref element_rect,
                ref attributes,
                styles: _,
                ref children,
            } => {
                let range = start_time..start_time + duration;
                if !range.contains(&target_time) {
                    return;
                }
                let target_time = target_time - start_time;
                match object_type {
                    ObjectType::Wrap => {
                        if children.is_empty() {
                            return;
                        }
                        let mut inner_renderer = rendering_context.create_renderer();
                        children.iter().for_each(|object| {
                            render_inner(
                                rendering_context,
                                &mut inner_renderer,
                                object,
                                target_time,
                                element_rect.width,
                                element_rect.height,
                            )
                        });
                        let child_image = inner_renderer.render(
                            element_rect.width.ceil() as u32,
                            element_rect.height.ceil() as u32,
                        );
                        let rendering_info =
                            element_rect.calc_rendering_info(outer_width, outer_height);
                        renderer.render_image(child_image, rendering_info);
                    }
                    ObjectType::Other(processor) => {
                        let child_image = (!children.is_empty()).then(|| {
                            let mut inner_renderer = rendering_context.create_renderer();
                            children.iter().for_each(|object| {
                                render_inner(
                                    rendering_context,
                                    &mut inner_renderer,
                                    object,
                                    target_time,
                                    element_rect.width,
                                    element_rect.height,
                                )
                            });
                            inner_renderer.render(
                                element_rect.width.ceil() as u32,
                                element_rect.height.ceil() as u32,
                            )
                        });
                        let result = processor.process(target_time, attributes, child_image);
                        let rendering_info =
                            element_rect.calc_rendering_info(outer_width, outer_height);
                        renderer.render_image(result, rendering_info);
                    }
                }
            }
            ObjectData::Text(_) => {}
        }
    }

    let mut renderer = rendering_context.create_renderer();
    render_inner(
        &mut rendering_context,
        &mut renderer,
        object,
        frame_number as f64 / fps as f64,
        resolution_x as f32,
        resolution_y as f32,
    );
    renderer.render(resolution_x, resolution_y)
}
