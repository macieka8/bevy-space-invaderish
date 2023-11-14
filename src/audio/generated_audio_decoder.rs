use bevy::audio::Source;
use std::time::Duration;

use super::AudioWaveType;
use crate::utils::wave::{saw_wave, square_wave};

/// This decoder is responsible for playing the audio,
/// and so stores data about the audio being played.
pub struct GeneratedAudioDecoder {
    carrier_frequency: f32,
    modulation_frequency: f32,
    frequency_diff: f32,
    // how far along one period the wave is
    current_progress: f32,
    // how much we move along the period every frame
    progress_per_frame: f32,
    sample_rate: u32,
    audio_wave_type: AudioWaveType,
}

impl GeneratedAudioDecoder {
    pub fn new(
        carrier_frequency: f32,
        modulation_frequency: f32,
        frequency_diff: f32,
        audio_wave_type: AudioWaveType,
    ) -> Self {
        // standard sample rate for most recordings
        let sample_rate = 44_100;
        GeneratedAudioDecoder {
            carrier_frequency,
            modulation_frequency,
            frequency_diff,
            current_progress: 0.,
            progress_per_frame: 1.0 / sample_rate as f32,
            sample_rate,
            audio_wave_type,
        }
    }
}

// The decoder must implement iterator so that it can implement `Decodable`.
impl Iterator for GeneratedAudioDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_progress += self.progress_per_frame;
        if self.current_progress > 2.0 / self.modulation_frequency {
            self.current_progress -= 2.0 / self.modulation_frequency;
        }
        let modulation_progress = self.current_progress * self.modulation_frequency;
        let carrier_progress = self.current_progress * self.carrier_frequency;

        // todo: fm waves or something idk
        let fm = self.frequency_diff / self.modulation_frequency
            * f32::sin(2.0 * std::f32::consts::PI + modulation_progress);

        Some(match self.audio_wave_type {
            AudioWaveType::Sine => f32::sin(2.0 * std::f32::consts::PI * carrier_progress * fm),
            AudioWaveType::Squre => square_wave((carrier_progress * fm) % 1.0),
            AudioWaveType::Saw => saw_wave((carrier_progress * fm) % 1.0),
        })
    }
}

// `Source` is what allows the audio source to be played by bevy.
// This trait provides information on the audio.
impl Source for GeneratedAudioDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
