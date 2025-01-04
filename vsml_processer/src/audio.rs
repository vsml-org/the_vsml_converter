use std::collections::HashMap;
use vsml_common_audio::Audio as VsmlAudio;
use vsml_core::schemas::ObjectProcessor;

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

        let samples = match spec.sample_format {
            hound::SampleFormat::Float => {
                reader
                    .samples::<f32>()
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap()
            }
            hound::SampleFormat::Int => {
                reader
                    .samples::<i32>()
                    .map(|s| (s.unwrap() as f64 / (1i64 << (spec.bits_per_sample - 1)) as f64) as f32)
                    .collect()
            }
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
