use clap::Parser;
use std::collections::HashMap;
use std::env;
use std::path::{self, PathBuf};
use std::sync::Arc;
use vsml_audio_mixer::MixingContextImpl;
use vsml_common_audio::Audio as VsmlAudio;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::ObjectProcessor;
use vsml_encoder::encode;
use vsml_image_renderer::RenderingContextImpl;
use vsml_iv_converter::convert;
use vsml_parser::{VSSLoader, parse};
use vsml_processor::audio::AudioProcessor;
use vsml_processor::image::ImageProcessor;
use vsml_processor::text::TextProcessor;
use vsml_processor::video::VideoProcessor;
use vsml_text_renderer::TextRendererContext;

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

fn get_gpu_device() -> (wgpu::Device, wgpu::Queue) {
    // GPUのdeviceとqueueを作成
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .unwrap();
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        label: None,
        memory_hints: Default::default(),
        experimental_features: Default::default(),
        trace: Default::default(),
    }))
    .unwrap();
    (device, queue)
}

fn main() {
    let args = Args::parse();

    let output_path = args.output_path.map(|path| path::absolute(&path).expect("Failed to get output absolute path"));
    let vsml_string = std::fs::read_to_string(&args.input_path).unwrap();
    env::set_current_dir(args.input_path.parent().unwrap()).expect("Failed to set current directory");
    let vsml = parse(&vsml_string, &VSSFileLoader).unwrap();
    let (device, queue) = get_gpu_device();
    let provider = HashMap::from([
        (
            "img".to_string(),
            Arc::new(ImageProcessor::new(device.clone(), queue.clone()))
                as Arc<dyn ObjectProcessor<VsmlImage, VsmlAudio>>,
        ),
        (
            "aud".to_string(),
            Arc::new(AudioProcessor) as Arc<dyn ObjectProcessor<VsmlImage, VsmlAudio>>,
        ),
        (
            "vid".to_string(),
            Arc::new(VideoProcessor::new(device.clone(), queue.clone()))
                as Arc<dyn ObjectProcessor<VsmlImage, VsmlAudio>>,
        ),
        (
            "txt".to_string(),
            Arc::new(TextProcessor::new(TextRendererContext::new(
                device.clone(),
                queue.clone(),
            ))) as Arc<dyn ObjectProcessor<VsmlImage, VsmlAudio>>,
        ),
    ]);
    let iv_data = convert(&vsml, &provider);

    let mut rendering_context = RenderingContextImpl::new(device.clone(), queue.clone());
    let mut mixing_context = MixingContextImpl::new();

    encode(
        iv_data,
        &mut rendering_context,
        &mut mixing_context,
        output_path.as_deref(),
        args.overwrite,
        device,
        queue,
    );
}
