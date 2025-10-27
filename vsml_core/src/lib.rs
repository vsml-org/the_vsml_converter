use crate::schemas::{ObjectData, ObjectType, ProcessorInput};

pub mod schemas;
#[cfg(test)]
mod tests;

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

pub struct ImageEffectStyle {}
pub struct AudioEffectStyle {}

/// rendererから見た左上の座標とサイズ
#[derive(Debug)]
pub struct RenderingInfo {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[cfg_attr(test, mockall::automock(type Image=tests::MockImage;))]
pub trait Renderer {
    type Image;
    fn render_image(&mut self, image: Self::Image, info: RenderingInfo);
    fn render_box(&mut self, property: Property, info: RenderingInfo);
    fn render(self, width: u32, height: u32) -> Self::Image;
}

#[cfg_attr(test, mockall::automock(type Image=tests::MockImage; type Renderer=MockRenderer;))]
pub trait RenderingContext {
    type Image;
    type Renderer: Renderer<Image = Self::Image>;
    fn create_renderer(&mut self) -> Self::Renderer;
    fn apply_style(&mut self, image: Self::Image, style: ImageEffectStyle) -> Self::Image;
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
    fn apply_style(&mut self, image: Self::Image, style: ImageEffectStyle) -> Self::Image {
        R::apply_style(self, image, style)
    }
}

pub fn render_frame_image<R, A>(
    &schemas::IVData {
        resolution_x,
        resolution_y,
        fps,
        sampling_rate: _,
        ref object,
    }: &schemas::IVData<R::Image, A>,
    frame_number: u32,
    mut rendering_context: R,
) -> R::Image
where
    R: RenderingContext,
{
    fn render_inner<R, A>(
        rendering_context: &mut R,
        renderer: &mut R::Renderer,
        object: &ObjectData<R::Image, A>,
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
                        // 子要素からTextDataを収集
                        let mut text_data_list: Vec<schemas::TextData> = Vec::new();
                        for child in children {
                            if let ObjectData::Text { data } = child {
                                text_data_list.extend(data.iter().cloned());
                            }
                        }

                        let input = if !text_data_list.is_empty() {
                            // txtタグの場合: TextDataを渡す
                            ProcessorInput::Text(text_data_list)
                        } else if !children.is_empty() {
                            // img, vidなどの場合: 子要素をレンダリング
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
                            let image = inner_renderer.render(
                                element_rect.width.ceil() as u32,
                                element_rect.height.ceil() as u32,
                            );
                            ProcessorInput::Image(image)
                        } else {
                            ProcessorInput::None
                        };

                        println!("[debug] target_time: {}", target_time);
                        let result = processor.process_image(target_time, attributes, input);
                        if let Some(result) = result {
                            let rendering_info =
                                element_rect.calc_rendering_info(outer_width, outer_height);
                            renderer.render_image(result, rendering_info);
                        }
                    }
                }
            }
            ObjectData::Text { .. } => {
                // TextDataは親要素のProcessorで処理される
            }
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

#[cfg_attr(test, mockall::automock(type Audio=tests::MockAudio;))]
pub trait Mixer {
    type Audio;
    /// offset_time is the time in seconds from the start of the audio
    fn mix_audio(&mut self, _audio: Self::Audio, offset_time: f64, duration: f64);
    fn mix(self, duration: f64) -> Self::Audio;
}

#[cfg_attr(test, mockall::automock(type Audio=tests::MockAudio; type Mixer=MockMixer;))]
pub trait MixingContext {
    type Audio;
    type Mixer: Mixer<Audio = Self::Audio>;
    fn create_mixer(&mut self, sampling_rate: u32) -> Self::Mixer;
    fn apply_style(&mut self, audio: Self::Audio, style: AudioEffectStyle) -> Self::Audio;
}

impl<M> MixingContext for &mut M
where
    M: MixingContext,
{
    type Audio = M::Audio;
    type Mixer = M::Mixer;

    fn create_mixer(&mut self, sampling_rate: u32) -> Self::Mixer {
        M::create_mixer(self, sampling_rate)
    }
    fn apply_style(&mut self, audio: Self::Audio, style: AudioEffectStyle) -> Self::Audio {
        M::apply_style(self, audio, style)
    }
}

pub fn mix_audio<M, I>(
    &schemas::IVData {
        ref object,
        sampling_rate,
        ..
    }: &schemas::IVData<I, M::Audio>,
    mut mixing_context: M,
) -> M::Audio
where
    M: MixingContext,
{
    fn mix_inner<M, I>(
        mixing_context: &mut M,
        mixer: &mut M::Mixer,
        object: &ObjectData<I, M::Audio>,
        sampling_rate: u32,
        ancestor_duration: f64,
    ) where
        M: MixingContext,
    {
        match object {
            &ObjectData::Element {
                object_type: ObjectType::Wrap,
                duration,
                start_time,
                ref children,
                ..
            } => {
                if children.is_empty() {
                    return;
                }
                let mut inner_mixer = mixing_context.create_mixer(sampling_rate);
                children.iter().for_each(|object| {
                    mix_inner(
                        mixing_context,
                        &mut inner_mixer,
                        object,
                        sampling_rate,
                        ancestor_duration.min(duration),
                    )
                });
                let child_audio = inner_mixer.mix(ancestor_duration.min(duration));
                mixer.mix_audio(child_audio, start_time, ancestor_duration.min(duration));
            }
            &ObjectData::Element {
                object_type: ObjectType::Other(ref processor),
                duration,
                start_time,
                ref attributes,
                ref children,
                ..
            } => {
                let child_audio = (!children.is_empty()).then(|| {
                    let mut inner_mixer = mixing_context.create_mixer(sampling_rate);
                    children.iter().for_each(|object| {
                        mix_inner(
                            mixing_context,
                            &mut inner_mixer,
                            object,
                            sampling_rate,
                            ancestor_duration.min(duration),
                        )
                    });
                    inner_mixer.mix(ancestor_duration.min(duration))
                });
                let result = processor.process_audio(attributes, child_audio);
                if let Some(result) = result {
                    mixer.mix_audio(result, start_time, ancestor_duration.min(duration));
                }
            }
            ObjectData::Text { .. } => {}
        }
    }

    let mut mixer = mixing_context.create_mixer(sampling_rate);
    if let &ObjectData::Element { duration, .. } = object {
        mix_inner(
            &mut mixing_context,
            &mut mixer,
            object,
            sampling_rate,
            duration,
        );
        mixer.mix(duration)
    } else {
        unreachable!()
    }
}
