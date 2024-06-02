mod iv_data;
mod iv_file;

use crate::utils;

pub fn convert_iv_data(input_path: String) -> Vec<iv_data::IVData> {
    let vsml_text = utils::read_file(&input_path);
    if iv_file::is_cache_enabled(&vsml_text) {
        iv_file::read_iv_file()
    } else {
        convert_iv_data_from_vsml_text(vsml_text)
    }
}

fn convert_iv_data_from_vsml_text(vsml_text: String) -> Vec<iv_data::IVData> {
    vec![]
}
