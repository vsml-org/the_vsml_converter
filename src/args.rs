use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub input_path: String,

    #[arg(short, long = "output")]
    pub output_path: Option<String>,

    #[arg(short = 'f', long = "frame")]
    pub preview_frame: Option<usize>,

    #[arg(long = "duration")]
    pub preview_duration: Option<usize>,

    #[arg(long)]
    pub overwrite: bool,
}
