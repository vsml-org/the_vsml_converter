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
pub struct Args {
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
    let mut args = Args::parse();
    if args.output_path.is_none() {
        args.output_path = if args.preview_frame.is_some() && args.preview_duration.is_none() {
            Some(String::from("preview.png"))
        } else {
            Some(String::from("video.mp4"))
        }
    }
    args
}
