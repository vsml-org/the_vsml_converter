use clap::Parser;
use quick_xml::{
    events::Event,
    reader::Reader
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input_path: String,

    #[arg(short, long = "output")]
    output_path: Option<String>,

    #[arg(short, long)]
    frame: Option<usize>,

    #[arg(long)]
    overwrite: bool,
}

fn convert_xml_with_validate(xml_path: String) {
    let mut xml_reader = Reader::from_file(xml_path).unwrap();
    xml_reader.trim_text(true);
    let mut buf = Vec::new();
    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                println!("start: {}", String::from_utf8(e.name().as_ref().to_vec()).unwrap());
                if e.attributes().count() > 0 {
                    println!("attribute:");
                }
                for res in e.attributes() {
                    let attribute = res.unwrap();
                    print!(" {}: ", String::from_utf8(attribute.key.as_ref().to_vec()).unwrap());
                    println!("{}", String::from_utf8(attribute.value.as_ref().to_vec()).unwrap());
                }
            },
            Ok(Event::Text(e)) => {
                println!("text: {}", String::from_utf8(e.to_vec()).unwrap());
            },
            Ok(Event::End(e)) => {
                println!("end: {}", String::from_utf8(e.name().as_ref().to_vec()).unwrap());
            },
            _ => (),
        }
    }
}

fn main() {
    let mut args = Args::parse();
    if let None = &args.output_path {
        args.output_path = match args.frame {
            Some(_) => Some(String::from("preview.png")),
            None => Some(String::from("video.mp4")),
        }
    }

    convert_xml_with_validate(args.input_path)
}
