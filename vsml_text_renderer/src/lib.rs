use cosmic_text::{Buffer, FontSystem, Metrics, Shaping, SwashCache};
use std::cell::RefCell;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::{RectSize, TextData};

pub struct TextRendererContext {
    _device: wgpu::Device,
    _queue: wgpu::Queue,
    font_system: RefCell<FontSystem>,
    swash_cache: RefCell<SwashCache>,
}

impl TextRendererContext {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();

        // TODO: システムフォントの自動検出と読み込み
        // - font-kitのSystemSource::new()を使用
        // - OSごとのフォントディレクトリをスキャン
        // - デフォルトフォントの登録
        // - font_familyで指定されたフォント名からフォントを検索

        Self {
            _device: device,
            _queue: queue,
            font_system: RefCell::new(font_system),
            swash_cache: RefCell::new(swash_cache),
        }
    }

    /// TextDataからテキストをレンダリング
    pub fn render_text(&self, text_data: &TextData) -> VsmlImage {
        let mut font_system = self.font_system.borrow_mut();
        let mut _swash_cache = self.swash_cache.borrow_mut();

        let TextData { text, style: _ } = text_data;

        // TODO: font-sizeをTextStyleDataから取得
        // 現状はデフォルト値を使用
        let font_size = 16.0;
        let line_height = 20.0;

        let mut buffer = Buffer::new(&mut *font_system, Metrics::new(font_size, line_height));

        // TODO: フォントファミリーの設定
        // - TextStyleData.font_familyからフォントを選択
        // - cosmic_text::Attrsを使用してフォント属性を設定
        // - Family::Name()でフォント名を指定

        // TODO: TextStyleDataにfont-weight, font-style等を追加した場合、Attrsに反映

        buffer.set_text(
            &mut *font_system,
            text,
            &cosmic_text::Attrs::new(),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut *font_system, false);

        // TODO: cosmic-textのSwashCacheを使ってラスタライズ
        // 1. buffer.layout_runs()で各行の情報を取得
        // 2. 各グリフをswash_cache.get_image()でラスタライズ
        // 3. RGBAバッファに描画（colorを適用）
        // 4. wgpu::Textureに変換
        //    - device.create_texture()でテクスチャ作成
        //    - queue.write_texture()でピクセルデータを書き込み

        todo!("cosmic-textのラスタライズ結果をwgpu::Textureに変換")
    }

    /// TextDataからサイズを計算
    pub fn calculate_text_size(&self, text_data: &[TextData]) -> RectSize {
        let mut font_system = self.font_system.borrow_mut();

        // TODO: 複数のTextDataに対応（現状は最初の要素のみ）
        let TextData { text, style: _ } = &text_data[0];

        // TODO: font-sizeをTextStyleDataから取得
        let font_size = 16.0;
        let line_height = 20.0;

        let mut buffer = Buffer::new(&mut *font_system, Metrics::new(font_size, line_height));

        // TODO: フォントファミリーの設定

        buffer.set_text(
            &mut *font_system,
            text,
            &cosmic_text::Attrs::new(),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut *font_system, false);

        let (width, height) = self.calculate_buffer_size(&buffer);

        RectSize { width, height }
    }

    fn calculate_buffer_size(&self, buffer: &Buffer) -> (f32, f32) {
        let (width, total_lines) = buffer
            .layout_runs()
            .fold((0.0f32, 0usize), |(max_width, lines), run| {
                (max_width.max(run.line_w), lines + 1)
            });

        let height = total_lines as f32 * buffer.metrics().line_height;

        (width, height)
    }
}

// iv_converter用トレイト実装
impl vsml_iv_converter::TextMetricsCalculator for TextRendererContext {
    fn calculate_text_size(&self, text_data: &[TextData]) -> RectSize {
        TextRendererContext::calculate_text_size(self, text_data)
    }
}

// vsml_core用トレイト実装
impl vsml_core::TextRenderer for TextRendererContext {
    type Image = VsmlImage;

    fn render_text(&mut self, text_data: &vsml_core::schemas::TextData) -> Self::Image {
        TextRendererContext::render_text(self, text_data)
    }
}
