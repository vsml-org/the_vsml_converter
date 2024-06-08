use roxmltree::Node;

use super::iv_data;

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
    iv_data::IVData::new(
        "1920x1080".to_string(),
        "60".to_string(),
        "44100".to_string(),
    )
    .expect("")
}
