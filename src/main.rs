use clap::Parser as _;
use vsmlc::{args, parse_xml};

fn main() {
    let mut args = args::Args::parse();
    if args.output_path.is_none() {
        args.output_path = match args.preview_frame {
            Some(_) => Some(String::from("preview.png")),
            None => Some(String::from("video.mp4")),
        }
    }

    parse_xml::convert_xml_with_validate(args.input_path);
}
