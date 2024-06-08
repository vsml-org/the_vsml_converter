enum ObjectType {
    Wrp,
    Vobj,
    Aobj,
    Tobj,
}

// styleタグから持ってきたstyleのstruct
pub trait StyleData {}

// 実際に変換するときに使用するStyleのtrait
trait Style {
    fn adapt_style(&self);
}

/// こういった感じにtraitを実装するstructを複数作っていく
struct OriginX {}

impl Style for OriginX {
    fn adapt_style(&self) {}
}
///

pub struct ObjectData {
    object_type: ObjectType,
    start_time: f64,
    duration: f64,
    src: Option<String>,
    text: Option<String>,
    styles: Vec<Box<dyn Style>>,
}

pub struct IVData {
    resolution_x: usize,
    resolution_y: usize,
    fps: f64,
    sampling: usize,
    objects: Vec<ObjectData>,
}

impl IVData {
    pub fn new(resolution: String, fps: String, sampling: String) -> Result<IVData, String> {
        let resolutions = resolution.split("x").collect::<Vec<&str>>();
        if resolutions.len() != 2 {
            return Err("resolution format is invalid".to_string());
        }
        let resolution_x = match resolutions[0].parse::<usize>() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        let resolution_y = match resolutions[1].parse::<usize>() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        let fps = match fps.parse::<f64>() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        let sampling = match sampling.parse::<usize>() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        Ok(IVData {
            resolution_x,
            resolution_y,
            fps,
            sampling,
            objects: vec![],
        })
    }
}
