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
    pub fn white() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}
static COLOR_MAP: phf::Map<&'static str, Color> = phf_map! {
    "aliceblue" => Color { r:240, g:248, b:255, a:255 },
    "antiquewhite" => Color { r:250, g:235, b:215, a:255 },
    "aqua" => Color { r:0, g:255, b:255, a:255 },
    "aquamarine" => Color { r:127, g:255, b:212, a:255 },
    "azure" => Color { r:240, g:255, b:255, a:255 },
    "beige" => Color { r:245, g:245, b:220, a:255 },
    "bisque" => Color { r:255, g:228, b:196, a:255 },
    "black" => Color { r:0, g:0, b:0, a:255 },
    "blanchedalmond" => Color { r:255, g:235, b:205, a:255 },
    "blue" => Color { r:0, g:0, b:255, a:255 },
    "blueviolet" => Color { r:138, g:43, b:226, a:255 },
    "brown" => Color { r:165, g:42, b:42, a:255 },
    "burlywood" => Color { r:222, g:184, b:135, a:255 },
    "cadetblue" => Color { r:95, g:158, b:160, a:255 },
    "chartreuse" => Color { r:127, g:255, b:0, a:255 },
    "chocolate" => Color { r:210, g:105, b:30, a:255 },
    "coral" => Color { r:255, g:127, b:80, a:255 },
    "cornflowerblue" => Color { r:100, g:149, b:237, a:255 },
    "cornsilk" => Color { r:255, g:248, b:220, a:255 },
    "crimson" => Color { r:220, g:20, b:60, a:255 },
    "cyan" => Color { r:0, g:255, b:255, a:255 },
    "darkblue" => Color { r:0, g:0, b:139, a:255 },
    "darkcyan" => Color { r:0, g:139, b:139, a:255 },
    "darkgoldenrod" => Color { r:184, g:134, b:11, a:255 },
    "darkgray" | "darkgrey" => Color { r:169, g:169, b:169, a:255 },
    "darkgreen" => Color { r:0, g:100, b:0, a:255 },
    "darkkhaki" => Color { r:189, g:183, b:107, a:255 },
    "darkmagenta" => Color { r:139, g:0, b:139, a:255 },
    "darkolivegreen" => Color { r:85, g:107, b:47, a:255 },
    "darkorange" => Color { r:255, g:140, b:0, a:255 },
    "darkorchid" => Color { r:153, g:50, b:204, a:255 },
    "darkred" => Color { r:139, g:0, b:0, a:255 },
    "darksalmon" => Color { r:233, g:150, b:122, a:255 },
    "darkseagreen" => Color { r:143, g:188, b:143, a:255 },
    "darkslateblue" => Color { r:72, g:61, b:139, a:255 },
    "darkslategray" | "darkslategrey" => Color { r:47, g:79, b:79, a:255 },
    "darkturquoise" => Color { r:0, g:206, b:209, a:255 },
    "darkviolet" => Color { r:148, g:0, b:211, a:255 },
    "deeppink" => Color { r:255, g:20, b:147, a:255 },
    "deepskyblue" => Color { r:0, g:191, b:255, a:255 },
    "dimgray" | "dimgrey" => Color { r:105, g:105, b:105, a:255 },
    "dodgerblue" => Color { r:30, g:144, b:255, a:255 },
    "firebrick" => Color { r:178, g:34, b:34, a:255 },
    "floralwhite" => Color { r:255, g:250, b:240, a:255 },
    "forestgreen" => Color { r:34, g:139, b:34, a:255 },
    "fuchsia" => Color { r:255, g:0, b:255, a:255 },
    "gainsboro" => Color { r:220, g:220, b:220, a:255 },
    "ghostwhite" => Color { r:248, g:248, b:255, a:255 },
    "gold" => Color { r:255, g:215, b:0, a:255 },
    "goldenrod" => Color { r:218, g:165, b:32, a:255 },
    "gray" | "grey" => Color { r:128, g:128, b:128, a:255 },
    "green" => Color { r:0, g:128, b:0, a:255 },
    "greenyellow" => Color { r:173, g:255, b:47, a:255 },
    "honeydew" => Color { r:240, g:255, b:240, a:255 },
    "hotpink" => Color { r:255, g:105, b:180, a:255 },
    "indianred" => Color { r:205, g:92, b:92, a:255 },
    "indigo" => Color { r:75, g:0, b:130, a:255 },
    "ivory" => Color { r:255, g:255, b:240, a:255 },
    "khaki" => Color { r:240, g:230, b:140, a:255 },
    "lavender" => Color { r:230, g:230, b:250, a:255 },
    "lavenderblush" => Color { r:255, g:240, b:245, a:255 },
    "lawngreen" => Color { r:124, g:252, b:0, a:255 },
    "lemonchiffon" => Color { r:255, g:250, b:205, a:255 },
    "lightblue" => Color { r:173, g:216, b:230, a:255 },
    "lightcoral" => Color { r:240, g:128, b:128, a:255 },
    "lightcyan" => Color { r:224, g:255, b:255, a:255 },
    "lightgoldenrodyellow" => Color { r:250, g:250, b:210, a:255 },
    "lightgreen" => Color { r:144, g:238, b:144, a:255 },
    "lightgrey" => Color { r:211, g:211, b:211, a:255 },
    "lightpink" => Color { r:255, g:182, b:193, a:255 },
    "lightsalmon" => Color { r:255, g:160, b:122, a:255 },
    "lightseagreen" => Color { r:32, g:178, b:170, a:255 },
    "lightskyblue" => Color { r:135, g:206, b:250, a:255 },
    "lightslategray" | "lightslategrey" => Color { r:119, g:136, b:153, a:255 },
    "lightsteelblue" => Color { r:176, g:196, b:222, a:255 },
    "lightyellow" => Color { r:255, g:255, b:224, a:255 },
    "lime" => Color { r:0, g:255, b:0, a:255 },
    "limegreen" => Color { r:50, g:205, b:50, a:255 },
    "linen" => Color { r:250, g:240, b:230, a:255 },
    "magenta" => Color { r:255, g:0, b:255, a:255 },
    "maroon" => Color { r:128, g:0, b:0, a:255 },
    "mediumaquamarine" => Color { r:102, g:205, b:170, a:255 },
    "mediumblue" => Color { r:0, g:0, b:205, a:255 },
    "mediumorchid" => Color { r:186, g:85, b:211, a:255 },
    "mediumpurple" => Color { r:147, g:112, b:216, a:255 },
    "mediumseagreen" => Color { r:60, g:179, b:113, a:255 },
    "mediumslateblue" => Color { r:123, g:104, b:238, a:255 },
    "mediumspringgreen" => Color { r:0, g:250, b:154, a:255 },
    "mediumturquoise" => Color { r:72, g:209, b:204, a:255 },
    "mediumvioletred" => Color { r:199, g:21, b:133, a:255 },
    "midnightblue" => Color { r:25, g:25, b:112, a:255 },
    "mintcream" => Color { r:245, g:255, b:250, a:255 },
    "mistyrose" => Color { r:255, g:228, b:225, a:255 },
    "moccasin" => Color { r:255, g:228, b:181, a:255 },
    "navajowhite" => Color { r:255, g:222, b:173, a:255 },
    "navy" => Color { r:0, g:0, b:128, a:255 },
    "oldlace" => Color { r:253, g:245, b:230, a:255 },
    "olive" => Color { r:128, g:128, b:0, a:255 },
    "olivedrab" => Color { r:107, g:142, b:35, a:255 },
    "orange" => Color { r:255, g:165, b:0, a:255 },
    "orangered" => Color { r:255, g:69, b:0, a:255 },
    "orchid" => Color { r:218, g:112, b:214, a:255 },
    "palegoldenrod" => Color { r:238, g:232, b:170, a:255 },
    "palegreen" => Color { r:152, g:251, b:152, a:255 },
    "paleturquoise" => Color { r:175, g:238, b:238, a:255 },
    "palevioletred" => Color { r:216, g:112, b:147, a:255 },
    "papayawhip" => Color { r:255, g:239, b:213, a:255 },
    "peachpuff" => Color { r:255, g:218, b:185, a:255 },
    "peru" => Color { r:205, g:133, b:63, a:255 },
    "pink" => Color { r:255, g:192, b:203, a:255 },
    "plum" => Color { r:221, g:160, b:221, a:255 },
    "powderblue" => Color { r:176, g:224, b:230, a:255 },
    "purple" => Color { r:128, g:0, b:128, a:255 },
    "red" => Color { r:255, g:0, b:0, a:255 },
    "rosybrown" => Color { r:188, g:143, b:143, a:255 },
    "royalblue" => Color { r:65, g:105, b:225, a:255 },
    "saddlebrown" => Color { r:139, g:69, b:19, a:255 },
    "salmon" => Color { r:250, g:128, b:114, a:255 },
    "sandybrown" => Color { r:244, g:164, b:96, a:255 },
    "seagreen" => Color { r:46, g:139, b:87, a:255 },
    "seashell" => Color { r:255, g:245, b:238, a:255 },
    "sienna" => Color { r:160, g:82, b:45, a:255 },
    "silver" => Color { r:192, g:192, b:192, a:255 },
    "skyblue" => Color { r:135, g:206, b:235, a:255 },
    "slateblue" => Color { r:106, g:90, b:205, a:255 },
    "slategray" | "slategrey" => Color { r:112, g:128, b:144, a:255 },
    "snow" => Color { r:255, g:250, b:250, a:255 },
    "springgreen" => Color { r:0, g:255, b:127, a:255 },
    "steelblue" => Color { r:70, g:130, b:180, a:255 },
    "tan" => Color { r:210, g:180, b:140, a:255 },
    "teal" => Color { r:0, g:128, b:128, a:255 },
    "thistle" => Color { r:216, g:191, b:216, a:255 },
    "tomato" => Color { r:255, g:99, b:71, a:255 },
    "turquoise" => Color { r:64, g:224, b:208, a:255 },
    "violet" => Color { r:238, g:130, b:238, a:255 },
    "wheat" => Color { r:245, g:222, b:179, a:255 },
    "white" => Color { r:255, g:255, b:255, a:255 },
    "whitesmoke" => Color { r:245, g:245, b:245, a:255 },
    "yellow" => Color { r:255, g:255, b:0, a:255 },
    "yellowgreen" => Color { r:154, g:205, b:50, a:255 },
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
                        Ok(Color { r, g, b, a: 255 })
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
                        Ok(Color { r, g, b, a: 255 })
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
                    Ok(Color { r, g, b, a: 255 })
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
    pub color: Option<Color>,
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
}
