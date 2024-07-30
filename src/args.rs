use clap::{error::ErrorKind, Error, Parser};

fn validate_duration(preview_duration: &str) -> Result<Option<usize>, Error> {
    if preview_duration == "0" {
        return Err(Error::new(ErrorKind::WrongNumberOfValues));
    }
    let preview_duration = match preview_duration.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(Error::new(ErrorKind::InvalidValue)),
    };
    Ok(Some(preview_duration))
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ArgsForParse {
    pub input_path: String,

    #[arg(short, long = "output")]
    pub output_path: Option<String>,

    #[arg(short = 'f', long = "frame")]
    pub preview_frame: Option<usize>,

    #[arg(long = "duration", value_parser=validate_duration)]
    pub preview_duration: Option<usize>,

    #[arg(long)]
    pub overwrite: bool,
}

pub struct Args {
    pub input_path: String,
    pub src_base_path: String,
    pub output_path: String,
    pub preview_frame: Option<usize>,
    pub preview_duration: Option<usize>,
    pub overwrite: bool,
}

/**
 * preview_frameとpreview_durationの関係
 * | preview_frame | preview_duration |     result     |
 * | ------------- | ---------------- | -------------- |
 * |          none |             none |  video(0~last) |
 * |             0 |             none |       image(0) |
 * |            10 |             none |      image(10) |
 * |          none |                0 |        failure |
 * |            10 |                0 |        failure |
 * |          none |               10 |     video(0~9) |
 * |            10 |               10 |   video(10~19) |
 * |          none |                1 |       video(0) |
 * |            10 |                1 |      video(10) |
 */

pub fn get_parsed_args() -> Args {
    let args = ArgsForParse::parse();

    let output_path = match args.output_path {
        Some(v) => v,
        None => {
            if args.preview_frame.is_some() && args.preview_duration.is_none() {
                String::from("preview.png")
            } else {
                String::from("video.mp4")
            }
        }
    };
    let base_path_index = args.input_path.rfind('/');
    let src_base_path = match base_path_index {
        Some(index) => args.input_path[..index + 1].to_string(),
        None => String::new(),
    };
    Args {
        input_path: args.input_path,
        src_base_path,
        output_path,
        preview_frame: args.preview_frame,
        preview_duration: args.preview_duration,
        overwrite: args.overwrite,
    }
}
