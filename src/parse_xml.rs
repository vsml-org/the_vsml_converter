use crate::definitions::{RectSize, VideoData};
use crate::format_style::get_style_from_vss_path;
use quick_xml::{events::Event, reader::Reader};
use std::{fs::File, io::BufReader};

pub fn get_style_data(xml_reader: &mut Reader<BufReader<File>>) {
    println!("start meta");
    let mut buf = Vec::new();
    let mut target_tag_name = "".to_string();
    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                if target_tag_name.is_empty() && tag_name == "style" {
                    match tag_name.as_str() {
                        "style" => {
                            for res in e.attributes() {
                                let attribute = res.unwrap();
                                let key =
                                    String::from_utf8(attribute.key.as_ref().to_vec()).unwrap();
                                let value =
                                    String::from_utf8(attribute.value.as_ref().to_vec()).unwrap();
                                if key == "src" {
                                    get_style_from_vss_path(value);
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            Ok(Event::Start(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                if tag_name == "style" {
                    target_tag_name = tag_name;
                }
            }
            Ok(Event::Text(e)) => {
                let content = String::from_utf8(e.to_vec()).unwrap();
                println!(" text: {content}");
            }
            Ok(Event::End(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                println!("end: {tag_name}");
                if tag_name == "meta" {
                    break;
                }
            }
            _ => (),
        }
    }
}

pub fn convert_cont(xml_reader: &mut Reader<BufReader<File>>) {
    println!("start cont");
    let mut buf = Vec::new();
    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                println!("exist: {tag_name}");
                if e.attributes().count() > 0 {
                    println!("attribute:");
                }
                for res in e.attributes() {
                    let attribute = res.unwrap();
                    print!(
                        " {}: ",
                        String::from_utf8(attribute.key.as_ref().to_vec()).unwrap()
                    );
                    println!(
                        "{}",
                        String::from_utf8(attribute.value.as_ref().to_vec()).unwrap()
                    );
                }
            }
            Ok(Event::Start(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                println!("start: {tag_name}");
                if e.attributes().count() > 0 {
                    println!(" attribute:");
                }
                for res in e.attributes() {
                    let attribute = res.unwrap();
                    print!(
                        "  {}: ",
                        String::from_utf8(attribute.key.as_ref().to_vec()).unwrap()
                    );
                    println!(
                        "{}",
                        String::from_utf8(attribute.value.as_ref().to_vec()).unwrap()
                    );
                }
            }
            Ok(Event::Text(e)) => {
                let content = String::from_utf8(e.to_vec()).unwrap();
                println!(" text: {content}");
            }
            Ok(Event::End(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                println!("end: {tag_name}");
                if tag_name == "cont" {
                    break;
                }
            }
            _ => (),
        }
    }
}

pub fn convert_xml_with_validate(xml_path: String) -> VideoData {
    let mut xml_reader = Reader::from_file(xml_path).unwrap();
    xml_reader.trim_text(true);
    let mut buf = Vec::new();
    let mut status = "".to_string();
    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                match tag_name.as_str() {
                    "vsml" => {
                        if status.is_empty() {
                            status = tag_name;
                        } else {
                            panic!();
                        }
                    }
                    "meta" => {
                        if status == "vsml" {
                            status = tag_name;
                            get_style_data(&mut xml_reader);
                        } else {
                            panic!();
                        }
                    }
                    "cont" => {
                        if status == "vsml" || status == "meta" {
                            status = tag_name;
                            convert_cont(&mut xml_reader);
                        } else {
                            panic!();
                        }
                    }
                    _ => {
                        panic!();
                    }
                }
            }
            Ok(Event::End(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                match tag_name.as_str() {
                    "vsml" => (),
                    _ => {
                        panic!();
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(Event::Decl(_)) => (),
            Err(x) => {
                println!("{:?}", x);
                break;
            }
            _ => {
                panic!();
            }
        }
    }
    VideoData::new(RectSize::from_resolution_str("1920x1080").unwrap(), 10.0)
}
