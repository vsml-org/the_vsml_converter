use core::panic;

use roxmltree::Node;

use super::iv_data::{self, NestedObject};
use crate::definitions::CONTENT_TAG;
use crate::ffmpeg::get_duration;

pub fn format_style_data_from_meta(meta_node: Option<&Node>) -> Vec<Box<dyn iv_data::StyleData>> {
    let style_list: Vec<Box<dyn iv_data::StyleData>> = vec![];
    match meta_node {
        Some(v) => style_list,
        None => style_list,
    }
}

pub fn convert_iv_data_from_cont(
    cont_node: &Node,
    style_data: Vec<Box<dyn iv_data::StyleData>>,
) -> iv_data::IVData {
    let nested_object = convert_object_from_node(cont_node, 0.0, &style_data);

    iv_data::IVData::new(
        "1920x1080".to_string(),
        "60".to_string(),
        "44100".to_string(),
        nested_object.convert_to_objects(),
    )
    .expect("")
}

fn convert_object_from_node(
    wrap_node: &Node,
    start_time: f64,
    style_data: &Vec<Box<dyn iv_data::StyleData>>,
) -> iv_data::NestedObject {
    let is_content_tag = CONTENT_TAG.contains(&wrap_node.tag_name().name());

    if is_content_tag {
        let src_path = wrap_node.attribute("src");
        let inner_text = wrap_node.text();
        match wrap_node.tag_name().name() {
            "vid" => {
                let Some(src_path) = src_path else {
                    panic!();
                };
                let prefix = "organized_sample/";
                let duration = get_duration(format!("{prefix}{src_path}").as_str());
                NestedObject {
                    object_type: iv_data::ObjectType::Vobj,
                    start_time,
                    duration,
                    src: Some(src_path.to_string()),
                    text: None,
                    styles: vec![],
                    children: vec![NestedObject {
                        object_type: iv_data::ObjectType::Vobj,
                        start_time,
                        duration,
                        src: Some(src_path.to_string()),
                        text: None,
                        styles: vec![],
                        children: vec![],
                    }],
                }
            }
            "aud" => {
                let Some(src_path) = src_path else {
                    panic!();
                };
                let prefix = "organized_sample/";
                let duration = get_duration(format!("{prefix}{src_path}").as_str());
                NestedObject {
                    object_type: iv_data::ObjectType::Aobj,
                    start_time,
                    duration,
                    src: Some(src_path.to_string()),
                    text: None,
                    styles: vec![],
                    children: vec![],
                }
            }
            "img" => {
                let Some(src_path) = src_path else {
                    panic!();
                };
                NestedObject {
                    object_type: iv_data::ObjectType::Vobj,
                    start_time,
                    duration: f64::INFINITY,
                    src: Some(src_path.to_string()),
                    text: None,
                    styles: vec![],
                    children: vec![],
                }
            }
            "txt" => {
                let Some(inner_text) = inner_text else {
                    panic!();
                };
                NestedObject {
                    object_type: iv_data::ObjectType::Vobj,
                    start_time,
                    duration: f64::INFINITY,
                    src: None,
                    text: Some(inner_text.to_string()),
                    styles: vec![],
                    children: vec![],
                }
            }
            _ => {
                panic!();
            }
        }
    } else {
        let is_parallel = ["prl", "cont", "layer"].contains(&wrap_node.tag_name().name());
        let mut children: Vec<NestedObject> = vec![];
        let mut duration: f64 = 0.0;
        if is_parallel {
            for child in wrap_node.children() {
                let converted_child = convert_object_from_node(&child, start_time, style_data);
                if converted_child.duration != f64::INFINITY {
                    duration = duration.max(converted_child.duration);
                }
                children.push(converted_child);
            }
            if duration != f64::INFINITY {
                for child in children.iter_mut() {
                    if child.duration != f64::INFINITY {
                        child.set_duration_recursive(duration)
                    }
                }
            }
        } else {
            for child in wrap_node.children() {
                let converted_child =
                    convert_object_from_node(&child, start_time + duration, style_data);
                let is_infinity = converted_child.duration == f64::INFINITY;
                duration += converted_child.duration;
                children.push(converted_child);
                if is_infinity {
                    break;
                }
            }
        }
        NestedObject {
            object_type: iv_data::ObjectType::Wrp,
            start_time,
            duration,
            src: None,
            text: None,
            styles: vec![],
            children,
        }
    }
}
