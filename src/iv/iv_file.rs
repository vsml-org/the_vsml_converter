use super::iv_data;

pub fn is_cache_enabled(vsml_text: &String) -> bool {
    false
}

pub fn read_iv_file() -> iv_data::IVData {
    iv_data::IVData::new(
        "1920x1080".to_string(),
        "60".to_string(),
        "44100".to_string(),
    )
    .expect("")
}
