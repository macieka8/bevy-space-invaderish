use bevy::audio::Decodable;
use bevy::audio::Source;
use bevy::reflect::TypePath;
use bevy::reflect::TypeUuid;
use bevy::utils::Duration;
use rodio::source::TakeDuration;

use super::generated_audio_decoder::GeneratedAudioDecoder;
use super::AudioFrequency;
use super::AudioWaveType;

/// This struct usually contains the data for the audio being played.
/// This is where data read from an audio file would be stored, for example.
/// Implementing `TypeUuid` will automatically implement `Asset`.
/// This allows the type to be registered as an asset.
#[derive(TypePath, TypeUuid)]
#[uuid = "3047abfb-e880-479d-9f8b-d5420026021c"]
pub struct GeneratedAudio {
    pub audio_wave_type: AudioWaveType,
    pub carrier_frequency: f32,
    pub modulation_frequency: f32,
    pub frequency_diff: f32,
}

impl From<AudioFrequency> for GeneratedAudio {
    fn from(value: AudioFrequency) -> Self {
        GeneratedAudio {
            audio_wave_type: value.audio_wave_type,
            carrier_frequency: value.carrier_freq,
            modulation_frequency: value.modulation_freq,
            frequency_diff: value.freq_diff,
        }
    }
}

impl Decodable for GeneratedAudio {
    type Decoder = TakeDuration<GeneratedAudioDecoder>;

    type DecoderItem = <GeneratedAudioDecoder as Iterator>::Item;

    fn decoder(&self) -> Self::Decoder {
        GeneratedAudioDecoder::new(
            self.carrier_frequency,
            self.modulation_frequency,
            self.frequency_diff,
            self.audio_wave_type,
        )
        .take_duration(Duration::from_secs_f32(2.0 / self.modulation_frequency))
    }
}
