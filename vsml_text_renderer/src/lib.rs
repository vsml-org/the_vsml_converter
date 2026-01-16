#[cfg(test)]
mod tests;

use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache, fontdb};
use std::sync::RwLock;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::{Color, RectSize, TextData, TextStyleData};

#[derive(Debug)]
struct TextBounds {
    left: i32,
    right: i32,
    top: f32,
    bottom: f32,
}

impl TextBounds {
    fn new() -> Self {
        TextBounds {
            left: i32::MAX,
            right: i32::MIN,
            top: 0.0,
            bottom: 0.0,
        }
    }
    fn width(&self) -> i32 {
        self.right.saturating_sub(self.left).max(0)
    }
    fn height(&self) -> f32 {
        (self.bottom - self.top).ceil().max(0.0)
    }
}

fn calculate_line_height_from_font(
    font_system: &FontSystem,
    families: &Vec<Family>,
    font_size: f32,
) -> f32 {
    let fallback_default = font_size * 1.25;

    let db = font_system.db();

    let face_id = db.query(&fontdb::Query {
        families,
        weight: fontdb::Weight::NORMAL,
        stretch: fontdb::Stretch::Normal,
        style: fontdb::Style::Normal,
    });
    let Some(face_id) = face_id else {
        return fallback_default;
    };

    db.with_face_data(face_id, |data, face_index| {
        let Ok(face) = ttf_parser::Face::parse(data, face_index) else {
            return fallback_default;
        };

        let ascent = face.ascender() as f32;
        let descent = face.descender() as f32;
        let line_gap = face.line_gap() as f32;
        let units_per_em = face.units_per_em() as f32;

        let scale = font_size / units_per_em;
        (ascent - descent + line_gap) * scale
    })
    .unwrap_or(fallback_default)
}

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

        // レイアウトを計算する
        let font_family = Self::get_font_family_from_style(style);
        let font_size = style.font_size;
        let line_height =
            calculate_line_height_from_font(&font_system, &vec![font_family], font_size);
        let mut buffer = Buffer::new(&mut font_system, Metrics::new(font_size, line_height));
        let attrs = Attrs::new().family(font_family);
        buffer.set_text(&mut font_system, text, &attrs, Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut font_system, false);

        // 描画サイズの取得
        let bounds = self.calculate_buffer_bounds(&mut font_system, &mut swash_cache, &buffer);
        let width = bounds.width() as u32;
        let height = bounds.height() as u32;

        // RGBAバッファを作成（透明で初期化）
        let mut rgba_buffer = vec![0u8; (width * height * 4) as usize];
        // バッファ内のピクセルの値を計算
        for layout_run in buffer.layout_runs() {
            for glyph in layout_run.glyphs.iter() {
                let physical_glyph = glyph.physical((0.0, layout_run.line_y), 1.0);
                let Some(image) = swash_cache.get_image(&mut font_system, physical_glyph.cache_key)
                else {
                    continue;
                };
                if image.placement.width == 0 || image.placement.height == 0 {
                    continue;
                }

                // 描画領域の左上を原点(0,0)とした相対座標に変換
                let glyph_x = physical_glyph.x + image.placement.left - bounds.left;
                let glyph_y = physical_glyph.y - image.placement.top - bounds.top as i32;

                // グリフの各ピクセルをRGBAバッファに描画
                for (pixel_y, row) in image
                    .data
                    .chunks(image.placement.width as usize)
                    .enumerate()
                {
                    for (pixel_x, &alpha) in row.iter().enumerate() {
                        let Some(x) = pixel_x.checked_add_signed(glyph_x as isize) else {
                            continue;
                        };
                        let Some(y) = pixel_y.checked_add_signed(glyph_y as isize) else {
                            continue;
                        };
                        if !((0..width as usize).contains(&x) && (0..height as usize).contains(&y))
                        {
                            continue;
                        }

                        let pixel_index = (y * width as usize + x) * 4;
                        // TODO: アルファブレンディング処理後で見直す
                        let alpha_f = alpha as f32 / 255.0;
                        let [dr, dg, db, da, ..] = &mut rgba_buffer[pixel_index..] else {
                            unreachable!();
                        };
                        let Color { r, g, b, a } = style.color;
                        *dr = ((r as f32 * alpha_f) as u8).max(*dr);
                        *dg = ((g as f32 * alpha_f) as u8).max(*dg);
                        *db = ((b as f32 * alpha_f) as u8).max(*db);
                        *da = ((a as f32 * alpha_f) as u8).max(*da);
                    }
                }
            }
        }

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
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
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
        let mut swash_cache = self.swash_cache.write().unwrap();

        // TODO: 複数のTextDataに対応（現状は最初の要素のみ）
        let TextData { text, style } = &text_data[0];

        // レイアウトを計算する
        let font_family = Self::get_font_family_from_style(style);
        let font_size = style.font_size;
        let line_height =
            calculate_line_height_from_font(&font_system, &vec![font_family], font_size);
        let mut buffer = Buffer::new(&mut font_system, Metrics::new(font_size, line_height));
        let attrs = Attrs::new().family(font_family);
        buffer.set_text(&mut font_system, text, &attrs, Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut font_system, false);

        // 描画サイズの取得
        let bounds = self.calculate_buffer_bounds(&mut font_system, &mut swash_cache, &buffer);

        RectSize {
            width: bounds.width() as f32,
            height: bounds.height(),
        }
    }

    /// Bufferからテキストの境界を計算
    fn calculate_buffer_bounds(
        &self,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        buffer: &Buffer,
    ) -> TextBounds {
        let mut bounds = TextBounds::new();

        // 1行辺りの処理
        for layout_run in buffer.layout_runs() {
            // テキスト領域内のその行のtop
            bounds.top = bounds.top.min(layout_run.line_top);
            // テキスト領域内のその行のtop + その行のheight
            bounds.bottom = bounds
                .bottom
                .max(layout_run.line_top + layout_run.line_height);

            // 1文字辺りの処理
            for glyph in layout_run.glyphs.iter() {
                let physical_glyph = glyph.physical((0.0, layout_run.line_y), 1.0);
                let Some(image) = swash_cache.get_image(font_system, physical_glyph.cache_key)
                else {
                    continue;
                };

                // テキスト領域内の1文字の領域のleft + その文字のleft開始位置
                let glyph_left = physical_glyph.x + image.placement.left;
                bounds.left = bounds.left.min(glyph_left);
                bounds.right = bounds.right.max(glyph_left + image.placement.width as i32);
            }
        }

        bounds
    }

    /// TextStyleからフォントファミリーを取得
    /// TODO: フォールバック機能とフォールバック先の標準フォントは未対応
    fn get_font_family_from_style(style: &TextStyleData) -> Family<'_> {
        if !style.font_family.is_empty() {
            Family::Name(&style.font_family[0])
        } else {
            Family::SansSerif
        }
    }
}
