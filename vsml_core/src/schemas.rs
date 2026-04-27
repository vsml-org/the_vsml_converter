use crate::ElementRect;
use phf::phf_map;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ColorParseError {
    UnknownMode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub const fn from(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const WHITE: Self = Self::from_rgb(255, 255, 255);
}
static COLOR_MAP: phf::Map<&'static str, Color> = phf_map! {
    "aliceblue" => Color::from_rgb(240, 248, 255),
    "antiquewhite" => Color::from_rgb(250, 235, 215),
    "aqua" => Color::from_rgb(0, 255, 255),
    "aquamarine" => Color::from_rgb(127, 255, 212),
    "azure" => Color::from_rgb(240, 255, 255),
    "beige" => Color::from_rgb(245, 245, 220),
    "bisque" => Color::from_rgb(255, 228, 196),
    "black" => Color::from_rgb(0, 0, 0),
    "blanchedalmond" => Color::from_rgb(255, 235, 205),
    "blue" => Color::from_rgb(0, 0, 255),
    "blueviolet" => Color::from_rgb(138, 43, 226),
    "brown" => Color::from_rgb(165, 42, 42),
    "burlywood" => Color::from_rgb(222, 184, 135),
    "cadetblue" => Color::from_rgb(95, 158, 160),
    "chartreuse" => Color::from_rgb(127, 255, 0),
    "chocolate" => Color::from_rgb(210, 105, 30),
    "coral" => Color::from_rgb(255, 127, 80),
    "cornflowerblue" => Color::from_rgb(100, 149, 237),
    "cornsilk" => Color::from_rgb(255, 248, 220),
    "crimson" => Color::from_rgb(220, 20, 60),
    "cyan" => Color::from_rgb(0, 255, 255),
    "darkblue" => Color::from_rgb(0, 0, 139),
    "darkcyan" => Color::from_rgb(0, 139, 139),
    "darkgoldenrod" => Color::from_rgb(184, 134, 11),
    "darkgray" | "darkgrey" => Color::from_rgb(169, 169, 169),
    "darkgreen" => Color::from_rgb(0, 100, 0),
    "darkkhaki" => Color::from_rgb(189, 183, 107),
    "darkmagenta" => Color::from_rgb(139, 0, 139),
    "darkolivegreen" => Color::from_rgb(85, 107, 47),
    "darkorange" => Color::from_rgb(255, 140, 0),
    "darkorchid" => Color::from_rgb(153, 50, 204),
    "darkred" => Color::from_rgb(139, 0, 0),
    "darksalmon" => Color::from_rgb(233, 150, 122),
    "darkseagreen" => Color::from_rgb(143, 188, 143),
    "darkslateblue" => Color::from_rgb(72, 61, 139),
    "darkslategray" | "darkslategrey" => Color::from_rgb(47, 79, 79),
    "darkturquoise" => Color::from_rgb(0, 206, 209),
    "darkviolet" => Color::from_rgb(148, 0, 211),
    "deeppink" => Color::from_rgb(255, 20, 147),
    "deepskyblue" => Color::from_rgb(0, 191, 255),
    "dimgray" | "dimgrey" => Color::from_rgb(105, 105, 105),
    "dodgerblue" => Color::from_rgb(30, 144, 255),
    "firebrick" => Color::from_rgb(178, 34, 34),
    "floralwhite" => Color::from_rgb(255, 250, 240),
    "forestgreen" => Color::from_rgb(34, 139, 34),
    "fuchsia" => Color::from_rgb(255, 0, 255),
    "gainsboro" => Color::from_rgb(220, 220, 220),
    "ghostwhite" => Color::from_rgb(248, 248, 255),
    "gold" => Color::from_rgb(255, 215, 0),
    "goldenrod" => Color::from_rgb(218, 165, 32),
    "gray" | "grey" => Color::from_rgb(128, 128, 128),
    "green" => Color::from_rgb(0, 128, 0),
    "greenyellow" => Color::from_rgb(173, 255, 47),
    "honeydew" => Color::from_rgb(240, 255, 240),
    "hotpink" => Color::from_rgb(255, 105, 180),
    "indianred" => Color::from_rgb(205, 92, 92),
    "indigo" => Color::from_rgb(75, 0, 130),
    "ivory" => Color::from_rgb(255, 255, 240),
    "khaki" => Color::from_rgb(240, 230, 140),
    "lavender" => Color::from_rgb(230, 230, 250),
    "lavenderblush" => Color::from_rgb(255, 240, 245),
    "lawngreen" => Color::from_rgb(124, 252, 0),
    "lemonchiffon" => Color::from_rgb(255, 250, 205),
    "lightblue" => Color::from_rgb(173, 216, 230),
    "lightcoral" => Color::from_rgb(240, 128, 128),
    "lightcyan" => Color::from_rgb(224, 255, 255),
    "lightgoldenrodyellow" => Color::from_rgb(250, 250, 210),
    "lightgreen" => Color::from_rgb(144, 238, 144),
    "lightgrey" => Color::from_rgb(211, 211, 211),
    "lightpink" => Color::from_rgb(255, 182, 193),
    "lightsalmon" => Color::from_rgb(255, 160, 122),
    "lightseagreen" => Color::from_rgb(32, 178, 170),
    "lightskyblue" => Color::from_rgb(135, 206, 250),
    "lightslategray" | "lightslategrey" => Color::from_rgb(119, 136, 153),
    "lightsteelblue" => Color::from_rgb(176, 196, 222),
    "lightyellow" => Color::from_rgb(255, 255, 224),
    "lime" => Color::from_rgb(0, 255, 0),
    "limegreen" => Color::from_rgb(50, 205, 50),
    "linen" => Color::from_rgb(250, 240, 230),
    "magenta" => Color::from_rgb(255, 0, 255),
    "maroon" => Color::from_rgb(128, 0, 0),
    "mediumaquamarine" => Color::from_rgb(102, 205, 170),
    "mediumblue" => Color::from_rgb(0, 0, 205),
    "mediumorchid" => Color::from_rgb(186, 85, 211),
    "mediumpurple" => Color::from_rgb(147, 112, 216),
    "mediumseagreen" => Color::from_rgb(60, 179, 113),
    "mediumslateblue" => Color::from_rgb(123, 104, 238),
    "mediumspringgreen" => Color::from_rgb(0, 250, 154),
    "mediumturquoise" => Color::from_rgb(72, 209, 204),
    "mediumvioletred" => Color::from_rgb(199, 21, 133),
    "midnightblue" => Color::from_rgb(25, 25, 112),
    "mintcream" => Color::from_rgb(245, 255, 250),
    "mistyrose" => Color::from_rgb(255, 228, 225),
    "moccasin" => Color::from_rgb(255, 228, 181),
    "navajowhite" => Color::from_rgb(255, 222, 173),
    "navy" => Color::from_rgb(0, 0, 128),
    "oldlace" => Color::from_rgb(253, 245, 230),
    "olive" => Color::from_rgb(128, 128, 0),
    "olivedrab" => Color::from_rgb(107, 142, 35),
    "orange" => Color::from_rgb(255, 165, 0),
    "orangered" => Color::from_rgb(255, 69, 0),
    "orchid" => Color::from_rgb(218, 112, 214),
    "palegoldenrod" => Color::from_rgb(238, 232, 170),
    "palegreen" => Color::from_rgb(152, 251, 152),
    "paleturquoise" => Color::from_rgb(175, 238, 238),
    "palevioletred" => Color::from_rgb(216, 112, 147),
    "papayawhip" => Color::from_rgb(255, 239, 213),
    "peachpuff" => Color::from_rgb(255, 218, 185),
    "peru" => Color::from_rgb(205, 133, 63),
    "pink" => Color::from_rgb(255, 192, 203),
    "plum" => Color::from_rgb(221, 160, 221),
    "powderblue" => Color::from_rgb(176, 224, 230),
    "purple" => Color::from_rgb(128, 0, 128),
    "red" => Color::from_rgb(255, 0, 0),
    "rosybrown" => Color::from_rgb(188, 143, 143),
    "royalblue" => Color::from_rgb(65, 105, 225),
    "saddlebrown" => Color::from_rgb(139, 69, 19),
    "salmon" => Color::from_rgb(250, 128, 114),
    "sandybrown" => Color::from_rgb(244, 164, 96),
    "seagreen" => Color::from_rgb(46, 139, 87),
    "seashell" => Color::from_rgb(255, 245, 238),
    "sienna" => Color::from_rgb(160, 82, 45),
    "silver" => Color::from_rgb(192, 192, 192),
    "skyblue" => Color::from_rgb(135, 206, 235),
    "slateblue" => Color::from_rgb(106, 90, 205),
    "slategray" | "slategrey" => Color::from_rgb(112, 128, 144),
    "snow" => Color::from_rgb(255, 250, 250),
    "springgreen" => Color::from_rgb(0, 255, 127),
    "steelblue" => Color::from_rgb(70, 130, 180),
    "tan" => Color::from_rgb(210, 180, 140),
    "teal" => Color::from_rgb(0, 128, 128),
    "thistle" => Color::from_rgb(216, 191, 216),
    "tomato" => Color::from_rgb(255, 99, 71),
    "turquoise" => Color::from_rgb(64, 224, 208),
    "violet" => Color::from_rgb(238, 130, 238),
    "wheat" => Color::from_rgb(245, 222, 179),
    "white" => Color::from_rgb(255, 255, 255),
    "whitesmoke" => Color::from_rgb(245, 245, 245),
    "yellow" => Color::from_rgb(255, 255, 0),
    "yellowgreen" => Color::from_rgb(154, 205, 50),
};

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(value: &str) -> Result<Color, Self::Err> {
        match value {
            v if v.starts_with('#') => {
                let Some(hex) = v.strip_prefix('#') else {
                    return Err(ColorParseError::UnknownMode);
                };
                let Ok(num_value) = u32::from_str_radix(hex, 16) else {
                    return Err(ColorParseError::UnknownMode);
                };
                match hex.len() {
                    3 => {
                        let r = (((num_value >> 8) & 0xF) * 0x11) as u8;
                        let g = (((num_value >> 4) & 0xF) * 0x11) as u8;
                        let b = ((num_value & 0xF) * 0x11) as u8;
                        Ok(Color::from_rgb(r, g, b))
                    }
                    4 => {
                        let r = (((num_value >> 12) & 0xF) * 0x11) as u8;
                        let g = (((num_value >> 8) & 0xF) * 0x11) as u8;
                        let b = (((num_value >> 4) & 0xF) * 0x11) as u8;
                        let a = ((num_value & 0xF) * 0x11) as u8;
                        Ok(Color { r, g, b, a })
                    }
                    6 => {
                        let r = ((num_value >> 16) & 0xFF) as u8;
                        let g = ((num_value >> 8) & 0xFF) as u8;
                        let b = (num_value & 0xFF) as u8;
                        Ok(Color::from_rgb(r, g, b))
                    }
                    8 => {
                        let r = ((num_value >> 24) & 0xFF) as u8;
                        let g = ((num_value >> 16) & 0xFF) as u8;
                        let b = ((num_value >> 8) & 0xFF) as u8;
                        let a = (num_value & 0xFF) as u8;
                        Ok(Color { r, g, b, a })
                    }
                    _ => Err(ColorParseError::UnknownMode),
                }
            }
            v if v.starts_with("rgb(") => {
                static RGB_REGEX: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new(r"^rgb\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)$").unwrap()
                });
                if let Some(caps) = RGB_REGEX.captures(v) {
                    let r = caps[1].parse().map_err(|_| ColorParseError::UnknownMode)?;
                    let g = caps[2].parse().map_err(|_| ColorParseError::UnknownMode)?;
                    let b = caps[3].parse().map_err(|_| ColorParseError::UnknownMode)?;
                    Ok(Color::from_rgb(r, g, b))
                } else {
                    Err(ColorParseError::UnknownMode)
                }
            }
            v if v.starts_with("rgba(") => {
                static RGBA_REGEX: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new(r"^rgba\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)$")
                        .unwrap()
                });
                if let Some(caps) = RGBA_REGEX.captures(v) {
                    let r = caps[1].parse().map_err(|_| ColorParseError::UnknownMode)?;
                    let g = caps[2].parse().map_err(|_| ColorParseError::UnknownMode)?;
                    let b = caps[3].parse().map_err(|_| ColorParseError::UnknownMode)?;
                    let a = caps[4].parse().map_err(|_| ColorParseError::UnknownMode)?;
                    Ok(Color { r, g, b, a })
                } else {
                    Err(ColorParseError::UnknownMode)
                }
            }
            v => COLOR_MAP
                .get(v)
                .cloned()
                .ok_or(ColorParseError::UnknownMode),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LayerMode {
    Multi,
    Single,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LayerModeParseError {
    UnknownMode,
}

impl FromStr for LayerMode {
    type Err = LayerModeParseError;

    fn from_str(value: &str) -> Result<LayerMode, Self::Err> {
        match value {
            "multi" => Ok(LayerMode::Multi),
            "single" => Ok(LayerMode::Single),
            _ => Err(LayerModeParseError::UnknownMode),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    Percent,
    Frame,
    Second,
    Fit,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Duration {
    Percent(f64),
    Frame(usize),
    Second(f64),
    Fit,
}

#[derive(Debug, PartialEq, Eq, Hash, Error)]
pub enum DurationParseError {
    #[error("number parse error")]
    NumberParseError,
    #[error("unknown unit")]
    UnknownUnit,
}

impl FromStr for Duration {
    type Err = DurationParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "fit" {
            Ok(Duration::Fit)
        } else if value == "0" {
            Ok(Duration::Frame(0))
        } else if let Some(value) = value.strip_suffix('s') {
            let val = value
                .parse()
                .map_err(|_| DurationParseError::NumberParseError)?;
            Ok(Duration::Second(val))
        } else if let Some(value) = value.strip_suffix('f') {
            let val = value
                .parse()
                .map_err(|_| DurationParseError::NumberParseError)?;
            Ok(Duration::Frame(val))
        } else if let Some(value) = value.strip_suffix('%') {
            let val = value
                .parse()
                .map_err(|_| DurationParseError::NumberParseError)?;
            Ok(Duration::Percent(val))
        } else {
            Err(DurationParseError::UnknownUnit)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AudioVolume {
    Percent(f64),
}

#[derive(Debug, PartialEq, Eq, Hash, Error)]
pub enum AudioVolumeParseError {
    #[error("number parse error")]
    NumberParseError,
    #[error("unknown unit")]
    UnknownUnit,
}

impl FromStr for AudioVolume {
    type Err = AudioVolumeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(value) = value.strip_suffix('%') {
            let val = value
                .parse()
                .map_err(|_| AudioVolumeParseError::NumberParseError)?;
            Ok(AudioVolume::Percent(val))
        } else {
            Err(AudioVolumeParseError::UnknownUnit)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Length {
    /// ピクセルの絶対値
    Pixel(f32),
    /// 動画解像度の幅からの相対値(CSSのvwに近い)
    ResolutionWidth(f32),
    /// 動画解像度の高さからの相対値(CSSのvhに近い)
    ResolutionHeight(f32),
    /// 親のLengthからの相対値
    Percent(f64),
}

#[derive(Debug, PartialEq, Eq, Hash, Error)]
pub enum LengthParseError {
    #[error("number parse error")]
    NumberParseError,
    #[error("unknown unit")]
    UnknownUnit,
}

impl FromStr for Length {
    type Err = LengthParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "0" {
            Ok(Length::Pixel(0.0))
        } else if let Some(value) = value.strip_suffix("px") {
            let val = value
                .parse()
                .map_err(|_| LengthParseError::NumberParseError)?;
            Ok(Length::Pixel(val))
        } else if let Some(value) = value.strip_suffix("rw") {
            let val = value
                .parse()
                .map_err(|_| LengthParseError::NumberParseError)?;
            Ok(Length::ResolutionWidth(val))
        } else if let Some(value) = value.strip_suffix("rh") {
            let val = value
                .parse()
                .map_err(|_| LengthParseError::NumberParseError)?;
            Ok(Length::ResolutionHeight(val))
        } else if let Some(value) = value.strip_suffix('%') {
            let val = value
                .parse()
                .map_err(|_| LengthParseError::NumberParseError)?;
            Ok(Length::Percent(val))
        } else {
            Err(LengthParseError::UnknownUnit)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Order {
    Sequence,
    Parallel,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OrderParseError {
    UnknownMode,
}
impl FromStr for Order {
    type Err = OrderParseError;

    fn from_str(value: &str) -> Result<Order, Self::Err> {
        match value {
            "sequence" => Ok(Order::Sequence),
            "parallel" => Ok(Order::Parallel),
            _ => Err(OrderParseError::UnknownMode),
        }
    }
}

/// font-familyのパース用のutil関数
pub fn parse_font_family(value: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escaped = false;

    for c in value.chars() {
        match c {
            '"' | '\'' if !escaped => {
                // クォートの開始/終了
                in_quotes = !in_quotes;
                // クォートは保存しない
            }
            '\\' if !escaped => {
                // エスケープ文字
                escaped = true;
            }
            ',' if !in_quotes && !escaped => {
                // カンマによる区切り（クォート外かつエスケープされていない場合のみ）
                if !current.trim().is_empty() {
                    result.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => {
                // エスケープされた文字の場合は、バックスラッシュを除去して文字のみ追加
                if escaped {
                    current.push(c);
                    escaped = false;
                } else {
                    current.push(c);
                }
            }
        }
    }

    // 最後の要素を追加
    if !current.trim().is_empty() {
        result.push(current.trim().to_string());
    }
    result
}

#[derive(Debug)]
pub enum ObjectType<I, A> {
    Wrap,
    Other(Arc<dyn ObjectProcessor<I, A>>),
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct RectSize {
    pub width: f32,
    pub height: f32,
}

impl RectSize {
    pub const fn new(width: f32, height: f32) -> RectSize {
        RectSize { width, height }
    }
    pub const ZERO: RectSize = RectSize::new(0.0, 0.0);
}

pub trait ObjectProcessor<I, A> {
    fn name(&self) -> &str;
    fn default_duration(&self, attributes: &HashMap<String, String>) -> f64;
    fn default_image_size(&self, attributes: &HashMap<String, String>) -> RectSize;
    fn has_default_image_size(&self) -> bool;
    fn calculate_text_size(&self, text_data: &[TextData]) -> RectSize;
    fn process_image(
        &self,
        render_sec: f64,
        attributes: &HashMap<String, String>,
        input: ProcessorInput<I>,
        element_rect: &ElementRect,
    ) -> Option<I>;
    fn process_audio(&self, attributes: &HashMap<String, String>, audio: Option<A>) -> Option<A>;
}

impl<I, A> Debug for dyn ObjectProcessor<I, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ObjectProcessor({})", self.name())
    }
}

#[derive(Debug, Clone)]
pub struct TextStyleData {
    pub color: Color,
    pub font_family: Vec<String>,
    pub font_size: f32,
    pub wrap_length: Option<f32>,
}
impl Default for TextStyleData {
    fn default() -> Self {
        TextStyleData {
            color: Color::WHITE,
            font_size: 32.0,
            // 環境によってプリインストールのフォントが変わるのでvsml_coreでは定義しない
            font_family: vec![],
            wrap_length: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextData {
    pub text: String,
    pub style: TextStyleData,
}

/// ObjectProcessorへの入力データ
/// 画像データと文字データは排他的（同時には存在しない）
#[derive(Debug)]
pub enum ProcessorInput<I> {
    /// 入力なし
    None,
    /// 画像入力
    Image(I),
    /// テキスト入力
    Text(Vec<TextData>),
}

/// Elementまたはテキスト1つに相当するデータ
#[derive(Debug)]
pub enum ObjectData<I, A> {
    Element {
        object_type: ObjectType<I, A>,
        /// 親エレメントからの相対開始時間(s)
        start_time: f64,
        /// エレメントが表示される時間(s)
        duration: f64,
        /// 音量（1.0 = 100%）
        audio_volume: f64,
        background_color: Option<Color>,
        attributes: HashMap<String, String>,
        /// エレメントの表示位置とサイズ
        /// x, yは親エレメントからの相対位置
        element_rect: ElementRect,
        children: Vec<ObjectData<I, A>>,
    },
    Text(Vec<TextData>),
}

#[derive(Debug)]
pub struct IVData<I, A> {
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub fps: u32,
    pub sampling_rate: u32,
    pub object: ObjectData<I, A>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_layer_mode() {
        assert_eq!("multi".parse::<LayerMode>(), Ok(LayerMode::Multi));
        assert_eq!("single".parse::<LayerMode>(), Ok(LayerMode::Single));
        assert_eq!(
            "unknown".parse::<LayerMode>(),
            Err(LayerModeParseError::UnknownMode)
        );
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!("fit".parse::<Duration>(), Ok(Duration::Fit));
        assert_eq!("0".parse::<Duration>(), Ok(Duration::Frame(0)));
        assert_eq!("1f".parse::<Duration>(), Ok(Duration::Frame(1)));
        assert_eq!("1s".parse::<Duration>(), Ok(Duration::Second(1.0)));
        assert_eq!("1.0s".parse::<Duration>(), Ok(Duration::Second(1.0)));
        assert_eq!("1%".parse::<Duration>(), Ok(Duration::Percent(1.0)));
        assert_eq!("1.0%".parse::<Duration>(), Ok(Duration::Percent(1.0)));
        assert_eq!(
            "1.0".parse::<Duration>(),
            Err(DurationParseError::UnknownUnit)
        );
        assert_eq!(
            "1".parse::<Duration>(),
            Err(DurationParseError::UnknownUnit)
        );
    }

    #[test]
    fn test_parse_order() {
        assert_eq!("sequence".parse::<Order>(), Ok(Order::Sequence));
        assert_eq!("parallel".parse::<Order>(), Ok(Order::Parallel));
        assert_eq!(
            "unknown".parse::<Order>(),
            Err(OrderParseError::UnknownMode)
        );
    }

    #[test]
    fn test_parse_length() {
        assert_eq!("100px".parse::<Length>(), Ok(Length::Pixel(100.0)));
        assert_eq!("50.5px".parse::<Length>(), Ok(Length::Pixel(50.5)));
        assert_eq!("50rw".parse::<Length>(), Ok(Length::ResolutionWidth(50.0)));
        assert_eq!(
            "25.5rw".parse::<Length>(),
            Ok(Length::ResolutionWidth(25.5))
        );
        assert_eq!("50rh".parse::<Length>(), Ok(Length::ResolutionHeight(50.0)));
        assert_eq!(
            "75.5rh".parse::<Length>(),
            Ok(Length::ResolutionHeight(75.5))
        );
        assert_eq!("100%".parse::<Length>(), Ok(Length::Percent(100.0)));
        assert_eq!("50.5%".parse::<Length>(), Ok(Length::Percent(50.5)));
        assert_eq!("100".parse::<Length>(), Err(LengthParseError::UnknownUnit));
        assert_eq!(
            "100vw".parse::<Length>(),
            Err(LengthParseError::UnknownUnit)
        );
        assert_eq!("0px".parse::<Length>(), Ok(Length::Pixel(0.0)));
        assert_eq!("0".parse::<Length>(), Ok(Length::Pixel(0.0)));
        assert_eq!("-100px".parse::<Length>(), Ok(Length::Pixel(-100.0)));
    }
}
