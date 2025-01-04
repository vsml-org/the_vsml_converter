use dasp::{interpolate::sinc::Sinc, ring_buffer, signal, slice::add_in_place, Signal};
use vsml_common_audio::Audio as VsmlAudio;
use vsml_core::AudioEffectStyle;

pub struct MixerImpl {
    audio: VsmlAudio,
}

pub struct MixingContextImpl {}

impl vsml_core::Mixer for MixerImpl {
    type Audio = VsmlAudio;

    fn mix_audio(&mut self, audio: Self::Audio, offset_time: f64, duration: f64) {
        let signal = signal::from_iter(audio.samples);

        let ring_buffer = ring_buffer::Fixed::from([[0.0, 0.0]; 100]);
        let sinc = Sinc::new(ring_buffer);
        let new_signal = signal.from_hz_to_hz(
            sinc,
            audio.sampling_rate as f64,
            self.audio.sampling_rate as f64,
        );
        let resampled_samples: Vec<_> = new_signal.until_exhausted().collect();

        let sampling_rate = self.audio.sampling_rate as f64;
        let offset_sample = (offset_time * sampling_rate) as usize;
        let duration_sample = (duration * sampling_rate) as usize;

        if offset_sample + duration_sample + 1 > self.audio.samples.len() {
            self.audio.samples.resize(
                offset_sample + duration_sample + 1,
                [0.0, 0.0],
            );
        }

        add_in_place(
            &mut self.audio.samples[offset_sample..][..=duration_sample],
            &resampled_samples[..=duration_sample],
        );
    }

    fn mix(mut self, duration: f64) -> Self::Audio {
        self.audio.samples.resize(
            (duration * self.audio.sampling_rate as f64) as usize,
            [0.0, 0.0],
        );
        self.audio
    }
}

impl MixingContextImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MixingContextImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl vsml_core::MixingContext for MixingContextImpl {
    type Audio = VsmlAudio;
    type Mixer = MixerImpl;

    fn create_mixer(&mut self, sampling_rate: u32) -> Self::Mixer {
        MixerImpl {
            audio: VsmlAudio {
                samples: Vec::new(),
                sampling_rate,
            },
        }
    }

    fn apply_style(&mut self, audio: Self::Audio, _style: AudioEffectStyle) -> Self::Audio {
        audio
    }
}
