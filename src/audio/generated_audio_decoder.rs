use bevy::audio::Source;
use std::time::Duration;

/// This decoder is responsible for playing the audio,
/// and so stores data about the audio being played.
pub struct GeneratedAudioDecoder {
    duration: f32,
    carrier_frequency: f32,
    // how far along one period the wave is
    time: f32,
    // how much we move along the period every frame
    progress_per_frame: f32,
    sample_rate: u32,

    main_function: fn(f32) -> f32,
    volume_function: fn(f32) -> f32,
    frequency_function: fn(f32) -> f32,
}

impl GeneratedAudioDecoder {
    pub fn new(
        carrier_frequency: f32,
        duration: f32,
        main_function: fn(f32) -> f32,
        volume_function: fn(f32) -> f32,
        frequency_function: fn(f32) -> f32,
    ) -> Self {
        // standard sample rate for most recordings
        let sample_rate = 44_100;
        GeneratedAudioDecoder {
            duration,
            carrier_frequency,
            sample_rate,
            time: 0.,
            progress_per_frame: 1.0 / sample_rate as f32,
            main_function,
            volume_function,
            frequency_function,
        }
    }
}

// The decoder must implement iterator so that it can implement `Decodable`.
impl Iterator for GeneratedAudioDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.time += self.progress_per_frame;
        let progress = self.time / self.duration;

        let volume = (self.volume_function)(progress);
        let scaled_freq = self.carrier_frequency * (self.frequency_function)(progress);

        Some(volume * (self.main_function)(self.time * scaled_freq))
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
