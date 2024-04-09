use std::{fs::File, io::Read};

use crate::definitions::StyleData;

pub fn get_style(vss_text: String, style_data: &mut StyleData) {
    println!("{vss_text}");
}

pub fn get_style_from_vss_path(vss_path: &str, style_data: &mut StyleData) {
    let mut vss_file = File::open(vss_path).unwrap();
    let mut vss_text = String::new();
    vss_file.read_to_string(&mut vss_text).unwrap();
    get_style(vss_text, style_data)
}
