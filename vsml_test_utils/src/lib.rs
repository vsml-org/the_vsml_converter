#[macro_export]
macro_rules! vrt_out_path {
    ($file_name:expr) => {{
        use std::env;
        use std::fs;
        use std::path::PathBuf;
        use std::thread;

        let current_thread = thread::current();
        let test_name = match current_thread.name() {
            Some(test_name) if test_name.contains("vrt") => test_name,
            test_name => panic!(
                "Test name must contain 'vrt' to use vrt_out_path macro. Current test name: {}",
                test_name.unwrap_or("unknown")
            ),
        };

        let path = if let Ok(vrt_root) = env::var("VSML_VRT_OUTPUT_PATH") {
            let mut p = PathBuf::from(vrt_root);
            p.push(env!("CARGO_CRATE_NAME"));
            p.extend(test_name.split("::"));
            p.push($file_name);

            fs::create_dir_all(p.parent().unwrap()).expect("Failed to create VRT output directory");
            p
        } else {
            PathBuf::from($file_name)
        };
        path
    }};
}
