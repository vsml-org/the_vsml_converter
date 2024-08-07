use std::fs;

use super::iv_data::{self, NestedObject};
use crate::definitions::CONTENT_TAG;
use crate::ffmpeg::get_duration;
use crate::iv::pre_style::{
    DurationFactory, PreStyleFactory, Selector, TimeMarginFactory, VSSData,
};

use roxmltree::Node;

pub fn convert_vss_data_from_meta(meta: Option<&Node>, base_path: &String) -> Vec<VSSData> {
    let meta_node = match meta {
        Some(v) => v,
        None => return vec![],
    };
    let mut style_list: Vec<VSSData> = vec![];
    for node in meta_node.children() {
        match node.attribute("src") {
            Some(v) => {
                let style_string = match fs::read_to_string(format!("{base_path}{v}")) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                style_list.append(&mut convert_to_vss_data(style_string.as_str()));
            }
            None => match node.text() {
                Some(style_str) => {
                    let replaced_style_str = style_str.replace('\n', "");
                    let replaced_style_str = replaced_style_str.trim();
                    if replaced_style_str.is_empty() {
                        continue;
                    }
                    style_list.append(&mut convert_to_vss_data(replaced_style_str));
                }
                None => continue,
            },
        };
    }
    style_list
}

fn convert_to_vss_data(style_str: &str) -> Vec<VSSData> {
    let pre_style_factory_list: Vec<Box<dyn PreStyleFactory>> =
        vec![Box::new(DurationFactory {}), Box::new(TimeMarginFactory {})];

    let mut vss_data_list = vec![];
    for single_style_str in style_str.split('}') {
        let format_single_style_str = single_style_str.trim();
        if format_single_style_str.is_empty() {
            continue;
        }
        let style_parts = format_single_style_str.split('{').collect::<Vec<&str>>();
        if style_parts.len() != 2 {
            return vss_data_list;
        }
        let selector_str = style_parts[0].trim();
        let selector_result = Selector::from_str(selector_str);

        let selector = match selector_result {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mut pre_style_list = vec![];
        for property_line in style_parts[1].split(';').collect::<Vec<&str>>() {
            let format_property_line = property_line.trim();
            if format_property_line.is_empty() {
                continue;
            }
            let property_and_value = property_line.split(':').collect::<Vec<&str>>();
            if property_and_value.len() != 2 {
                continue;
            }
            let property = property_and_value[0].trim();
            let value = property_and_value[1].trim();
            for pre_style_factory in &pre_style_factory_list {
                if pre_style_factory.check_property_name(property) {
                    pre_style_list
                        .append(&mut pre_style_factory.create_from_value(property, value));
                }
            }
        }

        vss_data_list.push(VSSData {
            selector,
            pre_style_list,
        })
    }
    vss_data_list
}

pub fn convert_iv_data_from_cont(
    cont_node: &Node,
    style_data: Vec<VSSData>,
    base_path: &String,
) -> iv_data::IVData {
    let nested_object = convert_object_from_node(cont_node, 0.0, &style_data, base_path);

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
    vss_data: &Vec<VSSData>,
    base_path: &String,
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
                let duration = get_duration(format!("{}{}", base_path, src_path).as_str());
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
                let duration = get_duration(format!("{base_path}{src_path}").as_str());
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
                let converted_child =
                    convert_object_from_node(&child, start_time, vss_data, base_path);
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
                    convert_object_from_node(&child, start_time + duration, vss_data, base_path);
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
