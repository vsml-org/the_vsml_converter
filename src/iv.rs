mod iv_data;
mod iv_file;
mod pre_style;
mod style;
mod vsml;

use roxmltree::Document;

use std::{fs, process};

pub fn convert_iv_data(input_path: String, base_path: &String) -> iv_data::IVData {
    let vsml_text = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(2);
        }
    };
    if iv_file::is_cache_enabled(&vsml_text) {
        iv_file::read_iv_file()
    } else {
        convert_iv_data_from_vsml_text(vsml_text, base_path)
    }
}

fn convert_iv_data_from_vsml_text(vsml_text: String, base_path: &String) -> iv_data::IVData {
    // xmlオブジェクトに変換
    let document = Document::parse(&vsml_text).expect("failed to cast dom tree");

    // metaとcontに分割
    let root_node = document.root_element();
    let root_children = root_node
        .children()
        .filter(|n| n.is_element())
        .collect::<Vec<_>>();
    let meta_node = root_children.iter().find(|n| n.has_tag_name("meta"));
    let cont_node = root_children
        .iter()
        .find(|n| n.has_tag_name("cont"))
        .expect("cont tag is not exist");

    // metaからstyle情報を集め、structを作る
    let vss_data = vsml::convert_vss_data_from_meta(meta_node, base_path);

    // contとstyleのstructを持って、IVDataを作る
    vsml::convert_iv_data_from_cont(cont_node, vss_data, base_path)
}
