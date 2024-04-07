use std::fmt;

pub const WRAP_TAGS: [&str; 6] = ["cont", "wrp", "seq", "prl", "rect", "layer"];
pub const CONTENT_TAGS: [&str; 4] = ["vid", "aud", "img", "txt"];

pub struct RectSize {
    pub width: usize,
    pub height: usize,
}

impl RectSize {
    pub fn new(width: usize, height: usize) -> RectSize {
        RectSize { width, height }
    }

    pub fn from_resolution_str(resolution_str: &str) -> Result<RectSize, fmt::Error> {
        match resolution_str.split_once('x') {
            Some((w, h)) => {
                let width = match w.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(fmt::Error),
                };
                let height = match h.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(fmt::Error),
                };
                Ok(RectSize { width, height })
            }
            None => Err(fmt::Error),
        }
    }
}

enum ContentType {
    Video,
    Audio,
    Image,
    Text,
}

pub struct Content {
    start_time: usize,
    end_time: usize,
    content_type: ContentType,
    exist_video: bool,
    exist_audio: bool,
    value: String,
}

pub struct VideoData {
    resolution: RectSize,
    fps: f64,
    contents: Vec<Content>,
}

impl VideoData {
    pub fn new(resolution: RectSize, fps: f64) -> VideoData {
        VideoData {
            resolution,
            fps,
            contents: Vec::new(),
        }
    }
}
