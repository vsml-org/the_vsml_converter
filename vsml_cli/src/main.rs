use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use vsml_audio_mixer::MixingContextImpl;
use vsml_common_audio::Audio as VsmlAudio;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::ObjectProcessor;
use vsml_encoder::encode;
use vsml_image_renderer::RenderingContextImpl;
use vsml_iv_converter::convert;
use vsml_parser::{parse, VSSLoader};
use vsml_processer::audio::AudioProcessor;
use vsml_processer::image::ImageProcessor;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input VSML file
    input_path: PathBuf,

    /// Path to the output file
    #[arg(short, long = "output")]
    output_path: Option<PathBuf>,

    /// Overwrite the output file if it already exists
    #[arg(long)]
    overwrite: bool,
}

struct VSSFileLoader;

impl VSSLoader for VSSFileLoader {
    type Err = std::io::Error;
    fn load(&self, path: &str) -> Result<String, Self::Err> {
        std::fs::read_to_string(path)
    }
}

fn main() {
    let args = Args::parse();

    let vsml_string = std::fs::read_to_string(args.input_path).unwrap();
    let vsml = parse(&vsml_string, &VSSFileLoader).unwrap();
    let iv_data = convert(
        &vsml,
        &HashMap::from([
            (
                "img".to_string(),
                Arc::new(ImageProcessor) as Arc<dyn ObjectProcessor<VsmlImage, VsmlAudio>>,
            ),
            (
                "aud".to_string(),
                Arc::new(AudioProcessor) as Arc<dyn ObjectProcessor<VsmlImage, VsmlAudio>>,
            ),
        ]),
    );

    let mut rendering_context = RenderingContextImpl::new();
    let mut mixing_context = MixingContextImpl::new();

    encode(
        iv_data,
        &mut rendering_context,
        &mut mixing_context,
        args.output_path.as_deref(),
        args.overwrite,
    );
}
