use std::collections::HashMap;
use vsml_common_audio::Audio as VsmlAudio;
use vsml_core::schemas::{ObjectProcessor, RectSize};

pub struct AudioProcessor;

impl<I> ObjectProcessor<I, VsmlAudio> for AudioProcessor {
    fn name(&self) -> &str {
        "audio"
    }

    fn default_duration(&self, attributes: &HashMap<String, String>) -> f64 {
        let src_path = attributes.get("src").unwrap();
        let reader = hound::WavReader::open(src_path).unwrap();
        reader.duration() as f64 / reader.spec().sample_rate as f64
    }

    fn default_image_size(&self, _attributes: &HashMap<String, String>) -> RectSize {
        RectSize::ZERO
    }

    fn process_image(
        &self,
        _: f64,
        _attributes: &HashMap<String, String>,
        _: Option<I>,
    ) -> Option<I> {
        None
    }

    fn process_audio(
        &self,
        attributes: &HashMap<String, String>,
        _audio: Option<VsmlAudio>,
    ) -> Option<VsmlAudio> {
        let src_path = attributes.get("src").unwrap();
        let mut reader = hound::WavReader::open(src_path).unwrap();
        let spec = reader.spec();

        // attributesから時間情報を取得（存在しない場合は音声ファイル全体を読み込む）
        let effective_duration = attributes
            .get("_effective_duration")
            .and_then(|s| s.parse::<f64>().ok());

        let samples_to_read = if let Some(duration) = effective_duration {
            // 時間情報がある場合は、+1秒のバッファを読み込む
            let max_samples = ((duration + 1.0) * spec.sample_rate as f64 * spec.channels as f64) as u32;
            reader.duration().min(max_samples)
        } else {
            reader.duration()
        };

        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader
                .samples::<f32>()
                .take(samples_to_read as usize)
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            hound::SampleFormat::Int => reader
                .samples::<i32>()
                .take(samples_to_read as usize)
                .map(|s| (s.unwrap() as f64 / (1i64 << (spec.bits_per_sample - 1)) as f64) as f32)
                .collect(),
        };
        let samples = samples
            .chunks(spec.channels as usize)
            .map(|chunk| match chunk {
                &[left, right, ..] => [left, right],
                &[mono] => [mono, mono],
                [] => unreachable!("channels must be greater than 0"),
            })
            .collect::<Vec<[f32; 2]>>();

        Some(VsmlAudio {
            samples,
            sampling_rate: spec.sample_rate,
        })
    }
}
