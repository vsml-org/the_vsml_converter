use crate::ElementRect;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::Arc;
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

#[derive(Debug)]
pub struct FontColor(u8, u8, u8, u8); // 左からrgbaの値(0-255)

impl FontColor {
    pub fn value(&self) -> (u8, u8, u8, u8) {
        return (self.0, self.1, self.2, self.3);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FontColorParseError {
    UnknownMode,
}

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
            v if v.starts_with("rgb(") && v.ends_with(')') => {
                let content = &v[4..v.len() - 1];
                let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
                if parts.len() == 3 {
                    let r = parts[0]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    let g = parts[1]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    let b = parts[2]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    Ok(FontColor(r, g, b, 255))
                } else {
                    Err(FontColorParseError::UnknownMode)
                }
            }
            v if v.starts_with("rgba(") && v.ends_with(')') => {
                let content = &v[5..v.len() - 1];
                let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
                if parts.len() == 4 {
                    let r = parts[0]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    let g = parts[1]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    let b = parts[2]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    let a = parts[3]
                        .parse()
                        .map_err(|_| FontColorParseError::UnknownMode)?;
                    Ok(FontColor(r, g, b, a))
                } else {
                    Err(FontColorParseError::UnknownMode)
                }
            }
            v => match v.to_lowercase().as_str() {
                "aliceblue" => Ok(FontColor(240, 248, 255, 255)),
                "antiquewhite" => Ok(FontColor(250, 235, 215, 255)),
                "aqua" => Ok(FontColor(0, 255, 255, 255)),
                "aquamarine" => Ok(FontColor(127, 255, 212, 255)),
                "azure" => Ok(FontColor(240, 255, 255, 255)),
                "beige" => Ok(FontColor(245, 245, 220, 255)),
                "bisque" => Ok(FontColor(255, 228, 196, 255)),
                "black" => Ok(FontColor(0, 0, 0, 255)),
                "blanchedalmond" => Ok(FontColor(255, 235, 205, 255)),
                "blue" => Ok(FontColor(0, 0, 255, 255)),
                "blueviolet" => Ok(FontColor(138, 43, 226, 255)),
                "brown" => Ok(FontColor(165, 42, 42, 255)),
                "burlywood" => Ok(FontColor(222, 184, 135, 255)),
                "cadetblue" => Ok(FontColor(95, 158, 160, 255)),
                "chartreuse" => Ok(FontColor(127, 255, 0, 255)),
                "chocolate" => Ok(FontColor(210, 105, 30, 255)),
                "coral" => Ok(FontColor(255, 127, 80, 255)),
                "cornflowerblue" => Ok(FontColor(100, 149, 237, 255)),
                "cornsilk" => Ok(FontColor(255, 248, 220, 255)),
                "crimson" => Ok(FontColor(220, 20, 60, 255)),
                "cyan" => Ok(FontColor(0, 255, 255, 255)),
                "darkblue" => Ok(FontColor(0, 0, 139, 255)),
                "darkcyan" => Ok(FontColor(0, 139, 139, 255)),
                "darkgoldenrod" => Ok(FontColor(184, 134, 11, 255)),
                "darkgray" | "darkgrey" => Ok(FontColor(169, 169, 169, 255)),
                "darkgreen" => Ok(FontColor(0, 100, 0, 255)),
                "darkkhaki" => Ok(FontColor(189, 183, 107, 255)),
                "darkmagenta" => Ok(FontColor(139, 0, 139, 255)),
                "darkolivegreen" => Ok(FontColor(85, 107, 47, 255)),
                "darkorange" => Ok(FontColor(255, 140, 0, 255)),
                "darkorchid" => Ok(FontColor(153, 50, 204, 255)),
                "darkred" => Ok(FontColor(139, 0, 0, 255)),
                "darksalmon" => Ok(FontColor(233, 150, 122, 255)),
                "darkseagreen" => Ok(FontColor(143, 188, 143, 255)),
                "darkslateblue" => Ok(FontColor(72, 61, 139, 255)),
                "darkslategray" | "darkslategrey" => Ok(FontColor(47, 79, 79, 255)),
                "darkturquoise" => Ok(FontColor(0, 206, 209, 255)),
                "darkviolet" => Ok(FontColor(148, 0, 211, 255)),
                "deeppink" => Ok(FontColor(255, 20, 147, 255)),
                "deepskyblue" => Ok(FontColor(0, 191, 255, 255)),
                "dimgray" | "dimgrey" => Ok(FontColor(105, 105, 105, 255)),
                "dodgerblue" => Ok(FontColor(30, 144, 255, 255)),
                "firebrick" => Ok(FontColor(178, 34, 34, 255)),
                "floralwhite" => Ok(FontColor(255, 250, 240, 255)),
                "forestgreen" => Ok(FontColor(34, 139, 34, 255)),
                "fuchsia" => Ok(FontColor(255, 0, 255, 255)),
                "gainsboro" => Ok(FontColor(220, 220, 220, 255)),
                "ghostwhite" => Ok(FontColor(248, 248, 255, 255)),
                "gold" => Ok(FontColor(255, 215, 0, 255)),
                "goldenrod" => Ok(FontColor(218, 165, 32, 255)),
                "gray" | "grey" => Ok(FontColor(128, 128, 128, 255)),
                "green" => Ok(FontColor(0, 128, 0, 255)),
                "greenyellow" => Ok(FontColor(173, 255, 47, 255)),
                "honeydew" => Ok(FontColor(240, 255, 240, 255)),
                "hotpink" => Ok(FontColor(255, 105, 180, 255)),
                "indianred" => Ok(FontColor(205, 92, 92, 255)),
                "indigo" => Ok(FontColor(75, 0, 130, 255)),
                "ivory" => Ok(FontColor(255, 255, 240, 255)),
                "khaki" => Ok(FontColor(240, 230, 140, 255)),
                "lavender" => Ok(FontColor(230, 230, 250, 255)),
                "lavenderblush" => Ok(FontColor(255, 240, 245, 255)),
                "lawngreen" => Ok(FontColor(124, 252, 0, 255)),
                "lemonchiffon" => Ok(FontColor(255, 250, 205, 255)),
                "lightblue" => Ok(FontColor(173, 216, 230, 255)),
                "lightcoral" => Ok(FontColor(240, 128, 128, 255)),
                "lightcyan" => Ok(FontColor(224, 255, 255, 255)),
                "lightgoldenrodyellow" => Ok(FontColor(250, 250, 210, 255)),
                "lightgreen" => Ok(FontColor(144, 238, 144, 255)),
                "lightgrey" => Ok(FontColor(211, 211, 211, 255)),
                "lightpink" => Ok(FontColor(255, 182, 193, 255)),
                "lightsalmon" => Ok(FontColor(255, 160, 122, 255)),
                "lightseagreen" => Ok(FontColor(32, 178, 170, 255)),
                "lightskyblue" => Ok(FontColor(135, 206, 250, 255)),
                "lightslategray" | "lightslategrey" => Ok(FontColor(119, 136, 153, 255)),
                "lightsteelblue" => Ok(FontColor(176, 196, 222, 255)),
                "lightyellow" => Ok(FontColor(255, 255, 224, 255)),
                "lime" => Ok(FontColor(0, 255, 0, 255)),
                "limegreen" => Ok(FontColor(50, 205, 50, 255)),
                "linen" => Ok(FontColor(250, 240, 230, 255)),
                "magenta" => Ok(FontColor(255, 0, 255, 255)),
                "maroon" => Ok(FontColor(128, 0, 0, 255)),
                "mediumaquamarine" => Ok(FontColor(102, 205, 170, 255)),
                "mediumblue" => Ok(FontColor(0, 0, 205, 255)),
                "mediumorchid" => Ok(FontColor(186, 85, 211, 255)),
                "mediumpurple" => Ok(FontColor(147, 112, 216, 255)),
                "mediumseagreen" => Ok(FontColor(60, 179, 113, 255)),
                "mediumslateblue" => Ok(FontColor(123, 104, 238, 255)),
                "mediumspringgreen" => Ok(FontColor(0, 250, 154, 255)),
                "mediumturquoise" => Ok(FontColor(72, 209, 204, 255)),
                "mediumvioletred" => Ok(FontColor(199, 21, 133, 255)),
                "midnightblue" => Ok(FontColor(25, 25, 112, 255)),
                "mintcream" => Ok(FontColor(245, 255, 250, 255)),
                "mistyrose" => Ok(FontColor(255, 228, 225, 255)),
                "moccasin" => Ok(FontColor(255, 228, 181, 255)),
                "navajowhite" => Ok(FontColor(255, 222, 173, 255)),
                "navy" => Ok(FontColor(0, 0, 128, 255)),
                "oldlace" => Ok(FontColor(253, 245, 230, 255)),
                "olive" => Ok(FontColor(128, 128, 0, 255)),
                "olivedrab" => Ok(FontColor(107, 142, 35, 255)),
                "orange" => Ok(FontColor(255, 165, 0, 255)),
                "orangered" => Ok(FontColor(255, 69, 0, 255)),
                "orchid" => Ok(FontColor(218, 112, 214, 255)),
                "palegoldenrod" => Ok(FontColor(238, 232, 170, 255)),
                "palegreen" => Ok(FontColor(152, 251, 152, 255)),
                "paleturquoise" => Ok(FontColor(175, 238, 238, 255)),
                "palevioletred" => Ok(FontColor(216, 112, 147, 255)),
                "papayawhip" => Ok(FontColor(255, 239, 213, 255)),
                "peachpuff" => Ok(FontColor(255, 218, 185, 255)),
                "peru" => Ok(FontColor(205, 133, 63, 255)),
                "pink" => Ok(FontColor(255, 192, 203, 255)),
                "plum" => Ok(FontColor(221, 160, 221, 255)),
                "powderblue" => Ok(FontColor(176, 224, 230, 255)),
                "purple" => Ok(FontColor(128, 0, 128, 255)),
                "red" => Ok(FontColor(255, 0, 0, 255)),
                "rosybrown" => Ok(FontColor(188, 143, 143, 255)),
                "royalblue" => Ok(FontColor(65, 105, 225, 255)),
                "saddlebrown" => Ok(FontColor(139, 69, 19, 255)),
                "salmon" => Ok(FontColor(250, 128, 114, 255)),
                "sandybrown" => Ok(FontColor(244, 164, 96, 255)),
                "seagreen" => Ok(FontColor(46, 139, 87, 255)),
                "seashell" => Ok(FontColor(255, 245, 238, 255)),
                "sienna" => Ok(FontColor(160, 82, 45, 255)),
                "silver" => Ok(FontColor(192, 192, 192, 255)),
                "skyblue" => Ok(FontColor(135, 206, 235, 255)),
                "slateblue" => Ok(FontColor(106, 90, 205, 255)),
                "slategray" | "slategrey" => Ok(FontColor(112, 128, 144, 255)),
                "snow" => Ok(FontColor(255, 250, 250, 255)),
                "springgreen" => Ok(FontColor(0, 255, 127, 255)),
                "steelblue" => Ok(FontColor(70, 130, 180, 255)),
                "tan" => Ok(FontColor(210, 180, 140, 255)),
                "teal" => Ok(FontColor(0, 128, 128, 255)),
                "thistle" => Ok(FontColor(216, 191, 216, 255)),
                "tomato" => Ok(FontColor(255, 99, 71, 255)),
                "turquoise" => Ok(FontColor(64, 224, 208, 255)),
                "violet" => Ok(FontColor(238, 130, 238, 255)),
                "wheat" => Ok(FontColor(245, 222, 179, 255)),
                "white" => Ok(FontColor(255, 255, 255, 255)),
                "whitesmoke" => Ok(FontColor(245, 245, 245, 255)),
                "yellow" => Ok(FontColor(255, 255, 0, 255)),
                "yellowgreen" => Ok(FontColor(154, 205, 50, 255)),
                _ => Err(FontColorParseError::UnknownMode),
            },
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
