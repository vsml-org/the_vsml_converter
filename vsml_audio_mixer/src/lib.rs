use vsml_common_audio::Audio as VsmlAudio;
use vsml_core::AudioEffectStyle;

pub struct MixerImpl {}

pub struct MixingContextImpl {}

impl vsml_core::Mixer for MixerImpl {
    type Audio = VsmlAudio;

    fn mix_audio(&mut self, _audio: Self::Audio) {}

    fn mix(self) -> Self::Audio {}
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

    fn create_mixer(&mut self) -> Self::Mixer {
        MixerImpl {}
    }

    fn apply_style(&mut self, audio: Self::Audio, _style: AudioEffectStyle) -> Self::Audio {
        audio
    }
}
