use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache};
use std::sync::RwLock;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::{RectSize, TextData};

pub struct TextRendererContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    font_system: RwLock<FontSystem>,
    swash_cache: RwLock<SwashCache>,
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
            device,
            queue,
            font_system: RwLock::new(font_system),
            swash_cache: RwLock::new(swash_cache),
        }
    }

    /// TextDataからテキストをレンダリング
    pub fn render_text(&self, text_data: &[TextData]) -> VsmlImage {
        let mut font_system = self.font_system.write().unwrap();
        let mut swash_cache = self.swash_cache.write().unwrap();

        // TODO: 複数のTextDataに対応（現状は最初の要素のみ）
        let TextData { text, style } = &text_data[0];

        // TODO: font-sizeをTextStyleDataから取得
        // 現状はデフォルト値を使用
        let font_size = 32.0;
        let line_height = 40.0;

        let mut buffer = Buffer::new(&mut font_system, Metrics::new(font_size, line_height));

        // フォントファミリーの設定
        // TODO: フォールバックフォントは未対応
        let font_family = if !style.font_family.is_empty() {
            Family::Name(&style.font_family[0])
        } else {
            Family::SansSerif
        };

        let attrs = Attrs::new().family(font_family);

        buffer.set_text(&mut font_system, text, &attrs, Shaping::Advanced);
        buffer.shape_until_scroll(&mut font_system, false);

        // 行の範囲とグリフの横幅を計算
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = 0.0f32;
        let mut max_y = 0.0f32;

        for run in buffer.layout_runs() {
            // 行の範囲を更新
            min_y = min_y.min(run.line_top);
            max_y = max_y.max(run.line_top + run.line_height);

            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((0.0, run.line_y), 1.0);

                if let Some(image) =
                    swash_cache.get_image(&mut font_system, physical_glyph.cache_key)
                {
                    let glyph_x = physical_glyph.x + image.placement.left;

                    min_x = min_x.min(glyph_x);
                    max_x = max_x.max(glyph_x + image.placement.width as i32);
                }
            }
        }

        // バッファサイズを決定（行の高さ全体を使用）
        let width = if max_x > min_x {
            (max_x - min_x) as u32
        } else {
            1
        };
        let height = (max_y - min_y).ceil() as u32;
        let offset_y = min_y as i32;

        // RGBAバッファを作成（透明で初期化）
        let mut rgba_buffer = vec![0u8; (width * height * 4) as usize];

        // テキストの色を取得（デフォルトは白）
        let text_color = style.color.unwrap_or((255, 255, 255, 255));

        // cosmic-textでテキストをラスタライズ（2回目のイテレーション）
        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((0.0, run.line_y), 1.0);

                if let Some(image) =
                    swash_cache.get_image(&mut font_system, physical_glyph.cache_key)
                {
                    let glyph_x = physical_glyph.x + image.placement.left - min_x;
                    let glyph_y = physical_glyph.y - image.placement.top - offset_y;

                    // グリフの各ピクセルをRGBAバッファに描画
                    for (pixel_y, row) in image
                        .data
                        .chunks(image.placement.width as usize)
                        .enumerate()
                    {
                        for (pixel_x, &alpha) in row.iter().enumerate() {
                            let x = pixel_x.checked_add_signed(glyph_x as isize).unwrap();
                            let y = pixel_y.checked_add_signed(glyph_y as isize).unwrap();

                            if (0..width as usize).contains(&x) && (0..height as usize).contains(&y)
                            {
                                let pixel_index = (y * width as usize + x) * 4;

                                // アルファブレンディング
                                let alpha_f = alpha as f32 / 255.0;
                                rgba_buffer[pixel_index] = ((text_color.0 as f32 * alpha_f) as u8)
                                    .max(rgba_buffer[pixel_index]);
                                rgba_buffer[pixel_index + 1] = ((text_color.1 as f32 * alpha_f)
                                    as u8)
                                    .max(rgba_buffer[pixel_index + 1]);
                                rgba_buffer[pixel_index + 2] = ((text_color.2 as f32 * alpha_f)
                                    as u8)
                                    .max(rgba_buffer[pixel_index + 2]);
                                rgba_buffer[pixel_index + 3] = ((text_color.3 as f32 * alpha_f)
                                    as u8)
                                    .max(rgba_buffer[pixel_index + 3]);
                            }
                        }
                    }
                }
            }
        }

        // wgpu::Textureに変換
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba_buffer,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        texture
    }

    /// TextDataからサイズを計算
    pub fn calculate_text_size(&self, text_data: &[TextData]) -> RectSize {
        let mut font_system = self.font_system.write().unwrap();

        // TODO: 複数のTextDataに対応（現状は最初の要素のみ）
        let TextData { text, style: _ } = &text_data[0];

        // TODO: font-sizeをTextStyleDataから取得
        let font_size = 32.0;
        let line_height = 40.0;

        let mut buffer = Buffer::new(&mut font_system, Metrics::new(font_size, line_height));

        // TODO: フォントファミリーの設定

        buffer.set_text(
            &mut font_system,
            text,
            &cosmic_text::Attrs::new(),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut font_system, false);

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
