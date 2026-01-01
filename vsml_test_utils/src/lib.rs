#[macro_export]
macro_rules! vrt_out_path {
    ($file_name:expr) => {{
        use std::env;
        use std::fs;
        use std::path::PathBuf;
        use std::thread;

        let test_name = thread::current().name().unwrap_or("unknown").to_string();
        if !test_name.contains("vrt") {
            panic!(
                "Test name must contain 'vrt' to use vrt_out_path macro. Current test name: {}",
                test_name
            );
        }

        let path = if let Ok(vrt_root) = env::var("VSML_VRT_OUTPUT_PATH") {
            let mut p = PathBuf::from(vrt_root);
            p.push(env!("CARGO_CRATE_NAME"));
            p.push(test_name.replace("::", "/"));
            p.push($file_name);

            if let Some(parent) = p.parent() {
                fs::create_dir_all(parent).expect("Failed to create VRT output directory");
            }
            p
        } else {
            PathBuf::from($file_name)
        };
        path
    }};
}
