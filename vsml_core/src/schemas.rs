use crate::ElementRect;
use phf::phf_map;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug, Clone, Default)]
pub struct StyleData {
    pub layer_mode: Option<LayerMode>,
    pub background_color: Option<Color>,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FontColor(u8, u8, u8, u8); // 左からrgbaの値(0-255)

impl FontColor {
    pub fn value(&self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FontColorParseError {
    UnknownMode,
}

static COLOR_MAP: phf::Map<&'static str, FontColor> = phf_map! {
    "aliceblue" => FontColor(240, 248, 255, 255),
    "antiquewhite" => FontColor(250, 235, 215, 255),
    "aqua" => FontColor(0, 255, 255, 255),
    "aquamarine" => FontColor(127, 255, 212, 255),
    "azure" => FontColor(240, 255, 255, 255),
    "beige" => FontColor(245, 245, 220, 255),
    "bisque" => FontColor(255, 228, 196, 255),
    "black" => FontColor(0, 0, 0, 255),
    "blanchedalmond" => FontColor(255, 235, 205, 255),
    "blue" => FontColor(0, 0, 255, 255),
    "blueviolet" => FontColor(138, 43, 226, 255),
    "brown" => FontColor(165, 42, 42, 255),
    "burlywood" => FontColor(222, 184, 135, 255),
    "cadetblue" => FontColor(95, 158, 160, 255),
    "chartreuse" => FontColor(127, 255, 0, 255),
    "chocolate" => FontColor(210, 105, 30, 255),
    "coral" => FontColor(255, 127, 80, 255),
    "cornflowerblue" => FontColor(100, 149, 237, 255),
    "cornsilk" => FontColor(255, 248, 220, 255),
    "crimson" => FontColor(220, 20, 60, 255),
    "cyan" => FontColor(0, 255, 255, 255),
    "darkblue" => FontColor(0, 0, 139, 255),
    "darkcyan" => FontColor(0, 139, 139, 255),
    "darkgoldenrod" => FontColor(184, 134, 11, 255),
    "darkgray" | "darkgrey" => FontColor(169, 169, 169, 255),
    "darkgreen" => FontColor(0, 100, 0, 255),
    "darkkhaki" => FontColor(189, 183, 107, 255),
    "darkmagenta" => FontColor(139, 0, 139, 255),
    "darkolivegreen" => FontColor(85, 107, 47, 255),
    "darkorange" => FontColor(255, 140, 0, 255),
    "darkorchid" => FontColor(153, 50, 204, 255),
    "darkred" => FontColor(139, 0, 0, 255),
    "darksalmon" => FontColor(233, 150, 122, 255),
    "darkseagreen" => FontColor(143, 188, 143, 255),
    "darkslateblue" => FontColor(72, 61, 139, 255),
    "darkslategray" | "darkslategrey" => FontColor(47, 79, 79, 255),
    "darkturquoise" => FontColor(0, 206, 209, 255),
    "darkviolet" => FontColor(148, 0, 211, 255),
    "deeppink" => FontColor(255, 20, 147, 255),
    "deepskyblue" => FontColor(0, 191, 255, 255),
    "dimgray" | "dimgrey" => FontColor(105, 105, 105, 255),
    "dodgerblue" => FontColor(30, 144, 255, 255),
    "firebrick" => FontColor(178, 34, 34, 255),
    "floralwhite" => FontColor(255, 250, 240, 255),
    "forestgreen" => FontColor(34, 139, 34, 255),
    "fuchsia" => FontColor(255, 0, 255, 255),
    "gainsboro" => FontColor(220, 220, 220, 255),
    "ghostwhite" => FontColor(248, 248, 255, 255),
    "gold" => FontColor(255, 215, 0, 255),
    "goldenrod" => FontColor(218, 165, 32, 255),
    "gray" | "grey" => FontColor(128, 128, 128, 255),
    "green" => FontColor(0, 128, 0, 255),
    "greenyellow" => FontColor(173, 255, 47, 255),
    "honeydew" => FontColor(240, 255, 240, 255),
    "hotpink" => FontColor(255, 105, 180, 255),
    "indianred" => FontColor(205, 92, 92, 255),
    "indigo" => FontColor(75, 0, 130, 255),
    "ivory" => FontColor(255, 255, 240, 255),
    "khaki" => FontColor(240, 230, 140, 255),
    "lavender" => FontColor(230, 230, 250, 255),
    "lavenderblush" => FontColor(255, 240, 245, 255),
    "lawngreen" => FontColor(124, 252, 0, 255),
    "lemonchiffon" => FontColor(255, 250, 205, 255),
    "lightblue" => FontColor(173, 216, 230, 255),
    "lightcoral" => FontColor(240, 128, 128, 255),
    "lightcyan" => FontColor(224, 255, 255, 255),
    "lightgoldenrodyellow" => FontColor(250, 250, 210, 255),
    "lightgreen" => FontColor(144, 238, 144, 255),
    "lightgrey" => FontColor(211, 211, 211, 255),
    "lightpink" => FontColor(255, 182, 193, 255),
    "lightsalmon" => FontColor(255, 160, 122, 255),
    "lightseagreen" => FontColor(32, 178, 170, 255),
    "lightskyblue" => FontColor(135, 206, 250, 255),
    "lightslategray" | "lightslategrey" => FontColor(119, 136, 153, 255),
    "lightsteelblue" => FontColor(176, 196, 222, 255),
    "lightyellow" => FontColor(255, 255, 224, 255),
    "lime" => FontColor(0, 255, 0, 255),
    "limegreen" => FontColor(50, 205, 50, 255),
    "linen" => FontColor(250, 240, 230, 255),
    "magenta" => FontColor(255, 0, 255, 255),
    "maroon" => FontColor(128, 0, 0, 255),
    "mediumaquamarine" => FontColor(102, 205, 170, 255),
    "mediumblue" => FontColor(0, 0, 205, 255),
    "mediumorchid" => FontColor(186, 85, 211, 255),
    "mediumpurple" => FontColor(147, 112, 216, 255),
    "mediumseagreen" => FontColor(60, 179, 113, 255),
    "mediumslateblue" => FontColor(123, 104, 238, 255),
    "mediumspringgreen" => FontColor(0, 250, 154, 255),
    "mediumturquoise" => FontColor(72, 209, 204, 255),
    "mediumvioletred" => FontColor(199, 21, 133, 255),
    "midnightblue" => FontColor(25, 25, 112, 255),
    "mintcream" => FontColor(245, 255, 250, 255),
    "mistyrose" => FontColor(255, 228, 225, 255),
    "moccasin" => FontColor(255, 228, 181, 255),
    "navajowhite" => FontColor(255, 222, 173, 255),
    "navy" => FontColor(0, 0, 128, 255),
    "oldlace" => FontColor(253, 245, 230, 255),
    "olive" => FontColor(128, 128, 0, 255),
    "olivedrab" => FontColor(107, 142, 35, 255),
    "orange" => FontColor(255, 165, 0, 255),
    "orangered" => FontColor(255, 69, 0, 255),
    "orchid" => FontColor(218, 112, 214, 255),
    "palegoldenrod" => FontColor(238, 232, 170, 255),
    "palegreen" => FontColor(152, 251, 152, 255),
    "paleturquoise" => FontColor(175, 238, 238, 255),
    "palevioletred" => FontColor(216, 112, 147, 255),
    "papayawhip" => FontColor(255, 239, 213, 255),
    "peachpuff" => FontColor(255, 218, 185, 255),
    "peru" => FontColor(205, 133, 63, 255),
    "pink" => FontColor(255, 192, 203, 255),
    "plum" => FontColor(221, 160, 221, 255),
    "powderblue" => FontColor(176, 224, 230, 255),
    "purple" => FontColor(128, 0, 128, 255),
    "red" => FontColor(255, 0, 0, 255),
    "rosybrown" => FontColor(188, 143, 143, 255),
    "royalblue" => FontColor(65, 105, 225, 255),
    "saddlebrown" => FontColor(139, 69, 19, 255),
    "salmon" => FontColor(250, 128, 114, 255),
    "sandybrown" => FontColor(244, 164, 96, 255),
    "seagreen" => FontColor(46, 139, 87, 255),
    "seashell" => FontColor(255, 245, 238, 255),
    "sienna" => FontColor(160, 82, 45, 255),
    "silver" => FontColor(192, 192, 192, 255),
    "skyblue" => FontColor(135, 206, 235, 255),
    "slateblue" => FontColor(106, 90, 205, 255),
    "slategray" | "slategrey" => FontColor(112, 128, 144, 255),
    "snow" => FontColor(255, 250, 250, 255),
    "springgreen" => FontColor(0, 255, 127, 255),
    "steelblue" => FontColor(70, 130, 180, 255),
    "tan" => FontColor(210, 180, 140, 255),
    "teal" => FontColor(0, 128, 128, 255),
    "thistle" => FontColor(216, 191, 216, 255),
    "tomato" => FontColor(255, 99, 71, 255),
    "turquoise" => FontColor(64, 224, 208, 255),
    "violet" => FontColor(238, 130, 238, 255),
    "wheat" => FontColor(245, 222, 179, 255),
    "white" => FontColor(255, 255, 255, 255),
    "whitesmoke" => FontColor(245, 245, 245, 255),
    "yellow" => FontColor(255, 255, 0, 255),
    "yellowgreen" => FontColor(154, 205, 50, 255),
};

impl FromStr for FontColor {
    type Err = FontColorParseError;

    fn from_str(value: &str) -> Result<FontColor, Self::Err> {
        match value {
            v if v.starts_with('#') => {
                let Some(hex) = v.strip_prefix('#') else {
                    return Err(FontColorParseError::UnknownMode);
                };
                let Ok(num_value) = u32::from_str_radix(hex, 16) else {
                    return Err(FontColorParseError::UnknownMode);
                };
                match hex.len() {
                    3 => {
                        let r = (((num_value >> 8) & 0xF) * 0x11) as u8;
                        let g = (((num_value >> 4) & 0xF) * 0x11) as u8;
                        let b = ((num_value & 0xF) * 0x11) as u8;
                        let a = 255;
                        Ok(FontColor(r, g, b, a))
                    }
                    4 => {
                        let r = (((num_value >> 12) & 0xF) * 0x11) as u8;
                        let g = (((num_value >> 8) & 0xF) * 0x11) as u8;
                        let b = (((num_value >> 4) & 0xF) * 0x11) as u8;
                        let a = ((num_value & 0xF) * 0x11) as u8;
                        Ok(FontColor(r, g, b, a))
                    }
                    6 => {
                        let r = ((num_value >> 16) & 0xFF) as u8;
                        let g = ((num_value >> 8) & 0xFF) as u8;
                        let b = (num_value & 0xFF) as u8;
                        let a = 255;
                        Ok(FontColor(r, g, b, a))
                    }
                    8 => {
                        let r = ((num_value >> 24) & 0xFF) as u8;
                        let g = ((num_value >> 16) & 0xFF) as u8;
                        let b = ((num_value >> 8) & 0xFF) as u8;
                        let a = (num_value & 0xFF) as u8;
                        Ok(FontColor(r, g, b, a))
                    }
                    _ => Err(FontColorParseError::UnknownMode),
                }
            }
            v if v.starts_with("rgb(") => {
                static RGB_REGEX: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new(r"^rgb\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)$").unwrap()
                });
                if let Some(caps) = RGB_REGEX.captures(v) {
                    let r = caps[1]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    let g = caps[2]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    let b = caps[3]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    return Ok(FontColor(r, g, b, 255));
                };
                Err(FontColorParseError::UnknownMode)
            }
            v if v.starts_with("rgba(") => {
                static RGBA_REGEX: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new(r"^rgba\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)$")
                        .unwrap()
                });
                if let Some(caps) = RGBA_REGEX.captures(v) {
                    let r = caps[1]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    let g = caps[2]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    let b = caps[3]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    let a = caps[4]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    return Ok(FontColor(r, g, b, a));
                }
                Err(FontColorParseError::UnknownMode)
            }
            v => COLOR_MAP
                .get(v)
                .cloned()
                .ok_or(FontColorParseError::UnknownMode),
        }
    }
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
    fn calculate_text_size(&self, text_data: &[TextData]) -> RectSize;
    fn process_image(
        &self,
        render_sec: f64,
        attributes: &HashMap<String, String>,
        input: ProcessorInput<I>,
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
    pub color: Option<(u8, u8, u8, u8)>,
    pub font_family: Vec<String>,
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
        attributes: HashMap<String, String>,
        /// エレメントの表示位置とサイズ
        /// x, yは親エレメントからの相対位置
        element_rect: ElementRect,
        styles: StyleData,
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
}
