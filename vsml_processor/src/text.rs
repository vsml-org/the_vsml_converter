use std::collections::HashMap;
use vsml_common_audio::Audio as VsmlAudio;
use vsml_common_image::Image as VsmlImage;
use vsml_core::{
    ElementRect,
    schemas::{ObjectProcessor, ProcessorInput, RectSize},
};
use vsml_text_renderer::TextRendererContext;

pub struct TextProcessor {
    text_renderer: TextRendererContext,
}

impl TextProcessor {
    pub fn new(text_renderer: TextRendererContext) -> Self {
        Self { text_renderer }
    }
}

impl ObjectProcessor<VsmlImage, VsmlAudio> for TextProcessor {
    fn name(&self) -> &str {
        "text"
    }

    fn default_duration(&self, _: &std::collections::HashMap<String, String>) -> f64 {
        f64::INFINITY
    }

    fn default_image_size(&self, _attributes: &HashMap<String, String>) -> RectSize {
        // サイズ計算は不要。convert_element_textでTextDataのrect_sizeが計算され、
        // 親要素のサイズに加算されるため
        RectSize::ZERO
    }

    fn has_default_image_size(&self) -> bool {
        false
    }

    fn calculate_text_size(&self, text_data: &[vsml_core::schemas::TextData]) -> RectSize {
        self.text_renderer.calculate_text_size(text_data)
    }

    fn process_image(
        &self,
        _time: f64,
        _attributes: &HashMap<String, String>,
        input: ProcessorInput<VsmlImage>,
        _element_rect: &ElementRect,
    ) -> Option<VsmlImage> {
        let ProcessorInput::Text(text_data_vec) = input else {
            return None;
        };
        if text_data_vec.is_empty() {
            return None;
        }
        // TODO: 複数のTextDataを適切にレイアウトして1つの画像に合成
        // 現状は最初のTextDataのみをレンダリング
        let image = self.text_renderer.render_text(&text_data_vec);
        Some(image)
    }

    fn process_audio(
        &self,
        _attributes: &HashMap<String, String>,
        _audio: Option<VsmlAudio>,
    ) -> Option<VsmlAudio> {
        None
    }
}
