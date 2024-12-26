use std::path::Path;
use std::process::Command;
use temp_dir::TempDir;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::{IVData, ObjectData};
use vsml_core::{render_frame_image, RenderingContext};

pub fn encode<R>(
    iv_data: IVData<R::Image>,
    mut rendering_context: R,
    output_path: Option<&Path>,
    overwrite: bool,
) where
    R: RenderingContext<Image = VsmlImage>,
{
    let ObjectData::Element { duration, .. } = iv_data.object else {
        panic!()
    };
    let whole_frames = duration * iv_data.fps as f64;

    let d = TempDir::new().unwrap();

    for f in 0..whole_frames.round() as u32 {
        let frame_image = render_frame_image(&iv_data, f, &mut rendering_context);
        let save_path = d.child(format!("frame_{}.png", f));
        frame_image.save(save_path).unwrap();
    }

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
        .arg(d.path().join("frame_%d.png"))
        .arg("-vcodec")
        .arg("libx264")
        .arg(output_path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
