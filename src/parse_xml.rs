use roxmltree::Document;

use crate::{
    definitions::{RectSize, StyleData, VideoData},
    format_content::format_video_data,
    format_style::{get_style, get_style_from_vss_path},
};
use std::{fs::File, io::Read};

pub fn convert_xml_with_validate(xml_path: String) -> VideoData {
    // documentの取得
    let mut xml_file = File::open(xml_path).expect("failed to open xml path");
    let mut xml_text = String::new();
    xml_file
        .read_to_string(&mut xml_text)
        .expect("failed to cast xml value to string");
    let document = Document::parse(&xml_text).expect("failed to cast dom tree");

    // root_tagの取得と検証
    let root_node = document.root_element();
    let root_children = root_node.children().filter(|n| n.is_element());
    let mut style_data = StyleData {};
    let mut video_data = None;
    for root_child in root_children {
        if root_child.has_tag_name("meta") {
            let meta_children = root_child.children().filter(|n| n.is_element());
            for meta_child in meta_children {
                if meta_child.has_tag_name("style") {
                    let style_path = meta_child.attribute("src");
                    if let Some(style_path) = style_path {
                        get_style_from_vss_path(style_path, &mut style_data);
                    } else {
                        let style_texts = meta_child.children().filter(|n| n.is_text());
                        for style_text in style_texts {
                            let vss_text = match style_text.text() {
                                Some(t) => t.to_string(),
                                None => continue,
                            };
                            get_style(vss_text, &mut style_data);
                        }
                    }
                }
            }
        }
        if root_child.has_tag_name("cont") {
            let resolution = root_child
                .attribute("resolution")
                .expect("resolution is not exist");
            let fps = root_child
                .attribute("fps")
                .expect("fps is not exist")
                .parse()
                .expect("fps value cannot cast to number");
            video_data = Some(VideoData::new(
                RectSize::from_resolution_str(resolution).unwrap(),
                fps,
            ));
            format_video_data(root_child, style_data, &mut video_data);
            break;
        }
    }
    video_data.unwrap()
}
