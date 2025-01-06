use std::path::Path;
use std::process::Command;
use temp_dir::TempDir;
use vsml_common_audio::Audio as VsmlAudio;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::{IVData, ObjectData};
use vsml_core::{mix_audio, render_frame_image, MixingContext, RenderingContext};

pub fn encode<R, M>(
    iv_data: IVData<R::Image, M::Audio>,
    mut rendering_context: R,
    mut mixing_context: M,
    output_path: Option<&Path>,
    overwrite: bool,
) where
    R: RenderingContext<Image = VsmlImage>,
    M: MixingContext<Audio = VsmlAudio>,
{
    let ObjectData::Element { duration, .. } = iv_data.object else {
        panic!()
    };
    assert!(duration.is_finite(), "動画時間が無限になっています");
    let whole_frames = duration * iv_data.fps as f64;

    let d = TempDir::new().unwrap();
    let d = d.path();

    for f in 0..whole_frames.round() as u32 {
        let frame_image = render_frame_image(&iv_data, f, &mut rendering_context);
        let save_path = d.join(format!("frame_{}.png", f));
        frame_image.save(save_path).unwrap();
    }

    let audio = mix_audio(&iv_data, &mut mixing_context);

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: audio.sampling_rate,
        sample_format: hound::SampleFormat::Float,
        bits_per_sample: 32,
    };
    let mut writer = hound::WavWriter::create(d.join("audio.wav"), spec).unwrap();
    audio.samples.iter().for_each(|s| {
        writer.write_sample(s[0]).unwrap();
        writer.write_sample(s[1]).unwrap();
    });
    writer.finalize().unwrap();

    let fps = iv_data.fps.to_string();
    let output_path = output_path.unwrap_or(Path::new("output.mp4"));

    let mut command = Command::new("ffmpeg");
    if overwrite {
        command.arg("-y");
    }
    command
        .arg("-r")
        .arg(&fps)
        .arg("-i")
        .arg(d.join("frame_%d.png"))
        .arg("-i")
        .arg(d.join("audio.wav"))
        .arg("-vcodec")
        .arg("libx264")
        .arg("-acodec")
        .arg("aac")
        .arg(output_path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
