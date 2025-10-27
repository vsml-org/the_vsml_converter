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
pub enum FontColor {
    /// 名前付きの色 (例: "red", "blue", "green")
    Named(String),
    Hex(String),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FontColorParseError {
    UnknownMode,
}

impl FontColor {
    pub fn to_rgba(&self) -> Result<(u8, u8, u8, u8), FontColorParseError> {
        match self {
            FontColor::Named(name) => match name.to_lowercase().as_str() {
                "aliceblue" => Ok((240, 248, 255, 255)),
                "antiquewhite" => Ok((250, 235, 215, 255)),
                "aqua" => Ok((0, 255, 255, 255)),
                "aquamarine" => Ok((127, 255, 212, 255)),
                "azure" => Ok((240, 255, 255, 255)),
                "beige" => Ok((245, 245, 220, 255)),
                "bisque" => Ok((255, 228, 196, 255)),
                "black" => Ok((0, 0, 0, 255)),
                "blanchedalmond" => Ok((255, 235, 205, 255)),
                "blue" => Ok((0, 0, 255, 255)),
                "blueviolet" => Ok((138, 43, 226, 255)),
                "brown" => Ok((165, 42, 42, 255)),
                "burlywood" => Ok((222, 184, 135, 255)),
                "cadetblue" => Ok((95, 158, 160, 255)),
                "chartreuse" => Ok((127, 255, 0, 255)),
                "chocolate" => Ok((210, 105, 30, 255)),
                "coral" => Ok((255, 127, 80, 255)),
                "cornflowerblue" => Ok((100, 149, 237, 255)),
                "cornsilk" => Ok((255, 248, 220, 255)),
                "crimson" => Ok((220, 20, 60, 255)),
                "cyan" => Ok((0, 255, 255, 255)),
                "darkblue" => Ok((0, 0, 139, 255)),
                "darkcyan" => Ok((0, 139, 139, 255)),
                "darkgoldenrod" => Ok((184, 134, 11, 255)),
                "darkgray" | "darkgrey" => Ok((169, 169, 169, 255)),
                "darkgreen" => Ok((0, 100, 0, 255)),
                "darkkhaki" => Ok((189, 183, 107, 255)),
                "darkmagenta" => Ok((139, 0, 139, 255)),
                "darkolivegreen" => Ok((85, 107, 47, 255)),
                "darkorange" => Ok((255, 140, 0, 255)),
                "darkorchid" => Ok((153, 50, 204, 255)),
                "darkred" => Ok((139, 0, 0, 255)),
                "darksalmon" => Ok((233, 150, 122, 255)),
                "darkseagreen" => Ok((143, 188, 143, 255)),
                "darkslateblue" => Ok((72, 61, 139, 255)),
                "darkslategray" | "darkslategrey" => Ok((47, 79, 79, 255)),
                "darkturquoise" => Ok((0, 206, 209, 255)),
                "darkviolet" => Ok((148, 0, 211, 255)),
                "deeppink" => Ok((255, 20, 147, 255)),
                "deepskyblue" => Ok((0, 191, 255, 255)),
                "dimgray" | "dimgrey" => Ok((105, 105, 105, 255)),
                "dodgerblue" => Ok((30, 144, 255, 255)),
                "firebrick" => Ok((178, 34, 34, 255)),
                "floralwhite" => Ok((255, 250, 240, 255)),
                "forestgreen" => Ok((34, 139, 34, 255)),
                "fuchsia" => Ok((255, 0, 255, 255)),
                "gainsboro" => Ok((220, 220, 220, 255)),
                "ghostwhite" => Ok((248, 248, 255, 255)),
                "gold" => Ok((255, 215, 0, 255)),
                "goldenrod" => Ok((218, 165, 32, 255)),
                "gray" | "grey" => Ok((128, 128, 128, 255)),
                "green" => Ok((0, 128, 0, 255)),
                "greenyellow" => Ok((173, 255, 47, 255)),
                "honeydew" => Ok((240, 255, 240, 255)),
                "hotpink" => Ok((255, 105, 180, 255)),
                "indianred" => Ok((205, 92, 92, 255)),
                "indigo" => Ok((75, 0, 130, 255)),
                "ivory" => Ok((255, 255, 240, 255)),
                "khaki" => Ok((240, 230, 140, 255)),
                "lavender" => Ok((230, 230, 250, 255)),
                "lavenderblush" => Ok((255, 240, 245, 255)),
                "lawngreen" => Ok((124, 252, 0, 255)),
                "lemonchiffon" => Ok((255, 250, 205, 255)),
                "lightblue" => Ok((173, 216, 230, 255)),
                "lightcoral" => Ok((240, 128, 128, 255)),
                "lightcyan" => Ok((224, 255, 255, 255)),
                "lightgoldenrodyellow" => Ok((250, 250, 210, 255)),
                "lightgreen" => Ok((144, 238, 144, 255)),
                "lightgrey" => Ok((211, 211, 211, 255)),
                "lightpink" => Ok((255, 182, 193, 255)),
                "lightsalmon" => Ok((255, 160, 122, 255)),
                "lightseagreen" => Ok((32, 178, 170, 255)),
                "lightskyblue" => Ok((135, 206, 250, 255)),
                "lightslategray" | "lightslategrey" => Ok((119, 136, 153, 255)),
                "lightsteelblue" => Ok((176, 196, 222, 255)),
                "lightyellow" => Ok((255, 255, 224, 255)),
                "lime" => Ok((0, 255, 0, 255)),
                "limegreen" => Ok((50, 205, 50, 255)),
                "linen" => Ok((250, 240, 230, 255)),
                "magenta" => Ok((255, 0, 255, 255)),
                "maroon" => Ok((128, 0, 0, 255)),
                "mediumaquamarine" => Ok((102, 205, 170, 255)),
                "mediumblue" => Ok((0, 0, 205, 255)),
                "mediumorchid" => Ok((186, 85, 211, 255)),
                "mediumpurple" => Ok((147, 112, 216, 255)),
                "mediumseagreen" => Ok((60, 179, 113, 255)),
                "mediumslateblue" => Ok((123, 104, 238, 255)),
                "mediumspringgreen" => Ok((0, 250, 154, 255)),
                "mediumturquoise" => Ok((72, 209, 204, 255)),
                "mediumvioletred" => Ok((199, 21, 133, 255)),
                "midnightblue" => Ok((25, 25, 112, 255)),
                "mintcream" => Ok((245, 255, 250, 255)),
                "mistyrose" => Ok((255, 228, 225, 255)),
                "moccasin" => Ok((255, 228, 181, 255)),
                "navajowhite" => Ok((255, 222, 173, 255)),
                "navy" => Ok((0, 0, 128, 255)),
                "oldlace" => Ok((253, 245, 230, 255)),
                "olive" => Ok((128, 128, 0, 255)),
                "olivedrab" => Ok((107, 142, 35, 255)),
                "orange" => Ok((255, 165, 0, 255)),
                "orangered" => Ok((255, 69, 0, 255)),
                "orchid" => Ok((218, 112, 214, 255)),
                "palegoldenrod" => Ok((238, 232, 170, 255)),
                "palegreen" => Ok((152, 251, 152, 255)),
                "paleturquoise" => Ok((175, 238, 238, 255)),
                "palevioletred" => Ok((216, 112, 147, 255)),
                "papayawhip" => Ok((255, 239, 213, 255)),
                "peachpuff" => Ok((255, 218, 185, 255)),
                "peru" => Ok((205, 133, 63, 255)),
                "pink" => Ok((255, 192, 203, 255)),
                "plum" => Ok((221, 160, 221, 255)),
                "powderblue" => Ok((176, 224, 230, 255)),
                "purple" => Ok((128, 0, 128, 255)),
                "red" => Ok((255, 0, 0, 255)),
                "rosybrown" => Ok((188, 143, 143, 255)),
                "royalblue" => Ok((65, 105, 225, 255)),
                "saddlebrown" => Ok((139, 69, 19, 255)),
                "salmon" => Ok((250, 128, 114, 255)),
                "sandybrown" => Ok((244, 164, 96, 255)),
                "seagreen" => Ok((46, 139, 87, 255)),
                "seashell" => Ok((255, 245, 238, 255)),
                "sienna" => Ok((160, 82, 45, 255)),
                "silver" => Ok((192, 192, 192, 255)),
                "skyblue" => Ok((135, 206, 235, 255)),
                "slateblue" => Ok((106, 90, 205, 255)),
                "slategray" | "slategrey" => Ok((112, 128, 144, 255)),
                "snow" => Ok((255, 250, 250, 255)),
                "springgreen" => Ok((0, 255, 127, 255)),
                "steelblue" => Ok((70, 130, 180, 255)),
                "tan" => Ok((210, 180, 140, 255)),
                "teal" => Ok((0, 128, 128, 255)),
                "thistle" => Ok((216, 191, 216, 255)),
                "tomato" => Ok((255, 99, 71, 255)),
                "turquoise" => Ok((64, 224, 208, 255)),
                "violet" => Ok((238, 130, 238, 255)),
                "wheat" => Ok((245, 222, 179, 255)),
                "white" => Ok((255, 255, 255, 255)),
                "whitesmoke" => Ok((245, 245, 245, 255)),
                "yellow" => Ok((255, 255, 0, 255)),
                "yellowgreen" => Ok((154, 205, 50, 255)),
                _ => Err(FontColorParseError::UnknownMode),
            },
            FontColor::Hex(hex) => {
                let hex = hex.trim_start_matches('#');
                match hex.len() {
                    3 | 4 => {
                        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                            .map_err(|_| FontColorParseError::UnknownMode)?;
                        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                            .map_err(|_| FontColorParseError::UnknownMode)?;
                        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                            .map_err(|_| FontColorParseError::UnknownMode)?;
                        let a = if hex.len() == 4 {
                            u8::from_str_radix(&hex[3..4].repeat(2), 16)
                                .map_err(|_| FontColorParseError::UnknownMode)?
                        } else {
                            255
                        };
                        Ok((r, g, b, a))
                    }
                    6 | 8 => {
                        let r = u8::from_str_radix(&hex[0..2], 16)
                            .map_err(|_| FontColorParseError::UnknownMode)?;
                        let g = u8::from_str_radix(&hex[2..4], 16)
                            .map_err(|_| FontColorParseError::UnknownMode)?;
                        let b = u8::from_str_radix(&hex[4..6], 16)
                            .map_err(|_| FontColorParseError::UnknownMode)?;
                        let a = if hex.len() == 8 {
                            u8::from_str_radix(&hex[6..8], 16)
                                .map_err(|_| FontColorParseError::UnknownMode)?
                        } else {
                            255
                        };
                        Ok((r, g, b, a))
                    }
                    _ => Err(FontColorParseError::UnknownMode),
                }
            }
            FontColor::Rgb(r, g, b) => Ok((*r, *g, *b, 255)),
            FontColor::Rgba(r, g, b, a) => Ok((*r, *g, *b, *a)),
        }
    }
}

impl FromStr for FontColor {
    type Err = FontColorParseError;

    fn from_str(value: &str) -> Result<FontColor, Self::Err> {
        match value {
            v if v.starts_with('#') => Ok(FontColor::Hex(v.to_string())),
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
                    Ok(FontColor::Rgb(r, g, b))
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
                    Ok(FontColor::Rgba(r, g, b, a))
                } else {
                    Err(FontColorParseError::UnknownMode)
                }
            }
            v => Ok(FontColor::Named(v.to_string())),
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
