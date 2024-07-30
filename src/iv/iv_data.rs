use crate::iv::style::Style;

#[derive(Debug)]
pub enum ObjectType {
    Wrp,
    Vobj,
    Aobj,
    Tobj,
}

pub struct NestedObject {
    pub object_type: ObjectType,
    pub start_time: f64,
    pub duration: f64,
    pub src: Option<String>,
    pub text: Option<String>,
    pub styles: Vec<Box<dyn Style>>,
    pub children: Vec<NestedObject>,
}

impl NestedObject {
    pub fn convert_to_objects(self) -> Vec<ObjectData> {
        let mut objects = vec![ObjectData::new(
            self.object_type,
            self.start_time,
            self.duration,
            self.src,
            self.text,
            self.styles,
        )];
        for child in self.children {
            objects.append(&mut child.convert_to_objects());
        }
        objects
    }
    pub fn set_duration_recursive(&mut self, duration: f64) {
        for child in self.children.iter_mut() {
            if child.duration != f64::INFINITY {
                child.duration = duration;
                child.set_duration_recursive(duration)
            }
        }
    }
}

#[derive(Debug)]
pub struct ObjectData {
    object_type: ObjectType,
    start_time: f64,
    duration: f64,
    src: Option<String>,
    text: Option<String>,
    styles: Vec<Box<dyn Style>>,
}

impl ObjectData {
    pub fn new(
        object_type: ObjectType,
        start_time: f64,
        duration: f64,
        src: Option<String>,
        text: Option<String>,
        styles: Vec<Box<dyn Style>>,
    ) -> ObjectData {
        ObjectData {
            object_type,
            start_time,
            duration,
            src,
            text,
            styles,
        }
    }
}

#[derive(Debug)]
pub struct IVData {
    resolution_x: usize,
    resolution_y: usize,
    fps: f64,
    sampling: usize,
    pub objects: Vec<ObjectData>,
}

impl IVData {
    pub fn new(
        resolution: String,
        fps: String,
        sampling: String,
        objects: Vec<ObjectData>,
    ) -> Result<IVData, String> {
        let resolutions = resolution.split('x').collect::<Vec<&str>>();
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
            objects,
        })
    }
}
