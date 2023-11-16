use bevy::audio::Decodable;
use bevy::audio::Source;
use bevy::ecs::system::Resource;
use bevy::reflect::TypePath;
use bevy::reflect::TypeUuid;
use bevy::utils::Duration;
use rodio::source::TakeDuration;

use super::generated_audio_decoder::GeneratedAudioDecoder;

/// This struct usually contains the data for the audio being played.
/// This is where data read from an audio file would be stored, for example.
/// Implementing `TypeUuid` will automatically implement `Asset`.
/// This allows the type to be registered as an asset.
#[derive(TypePath, TypeUuid, Resource)]
#[uuid = "3047abfb-e880-479d-9f8b-d5420026021c"]
pub struct GeneratedAudio {
    pub carrier_frequency: f32,
    pub duration: f32,
    pub main_function: fn(f32) -> f32,
    pub volume_function: fn(f32) -> f32,
    pub frequency_function: fn(f32) -> f32,
}

impl Decodable for GeneratedAudio {
    type Decoder = TakeDuration<GeneratedAudioDecoder>;

    type DecoderItem = <GeneratedAudioDecoder as Iterator>::Item;

    fn decoder(&self) -> Self::Decoder {
        GeneratedAudioDecoder::new(
            self.carrier_frequency,
            self.duration,
            self.main_function,
            self.volume_function,
            self.frequency_function,
        )
        .take_duration(Duration::from_secs_f32(self.duration))
    }
}
