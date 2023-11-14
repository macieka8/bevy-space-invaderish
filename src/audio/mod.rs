use crate::enemy::components::EnemyShootEvent;
use crate::player::PlayerShootEvent;
use crate::GameSet;
use bevy::audio::AddAudioSource;
use bevy::prelude::*;

use crate::audio::generated_audio::GeneratedAudio;

mod generated_audio;
mod generated_audio_decoder;

#[derive(Copy, Clone)]
pub enum AudioWaveType {
    Sine,
    Squre,
    Saw,
}

#[derive(Resource)]
pub struct AudioFrequency {
    pub audio_wave_type: AudioWaveType,
    pub carrier_freq: f32,
    pub modulation_freq: f32,
    pub freq_diff: f32,
}

const PLAYER_ATTACK_SOUND: AudioFrequency = AudioFrequency {
    audio_wave_type: AudioWaveType::Sine,
    carrier_freq: 300.0,
    modulation_freq: 9.0,
    freq_diff: 30.0,
};

const ENEMY_ATTACK_SOUND: AudioFrequency = AudioFrequency {
    audio_wave_type: AudioWaveType::Squre,
    carrier_freq: 100.0,
    modulation_freq: 12.0,
    freq_diff: 30.0,
};

pub struct AudioPlayerPlugin;

impl Plugin for AudioPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<GeneratedAudio>()
            .insert_resource::<AudioFrequency>(PLAYER_ATTACK_SOUND)
            .add_systems(
                Update,
                (
                    sound_tester.in_set(GameSet::Input),
                    player_shoot_audio,
                    enemy_shoot_audio,
                ),
            );
    }
}

fn player_shoot_audio(
    mut commands: Commands,
    mut assets: ResMut<Assets<GeneratedAudio>>,
    mut ev_player_shoot: EventReader<PlayerShootEvent>,
) {
    for _ev in ev_player_shoot.iter() {
        commands.spawn(AudioSourceBundle {
            source: assets.add(GeneratedAudio::from(PLAYER_ATTACK_SOUND)),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}

fn enemy_shoot_audio(
    mut commands: Commands,
    mut assets: ResMut<Assets<GeneratedAudio>>,
    mut ev_enemy_shoot: EventReader<EnemyShootEvent>,
) {
    for _ev in ev_enemy_shoot.iter() {
        commands.spawn(AudioSourceBundle {
            source: assets.add(GeneratedAudio::from(ENEMY_ATTACK_SOUND)),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}

fn sound_tester(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut assets: ResMut<Assets<GeneratedAudio>>,
    mut audio_freq: ResMut<AudioFrequency>,
) {
    if keyboard_input.just_pressed(KeyCode::Numpad7) {
        audio_freq.carrier_freq -= 10.0;
    } else if keyboard_input.just_pressed(KeyCode::Numpad8) {
        audio_freq.carrier_freq += 10.0;
    } else if keyboard_input.just_pressed(KeyCode::Numpad4) {
        audio_freq.modulation_freq -= 0.1;
    } else if keyboard_input.just_pressed(KeyCode::Numpad5) {
        audio_freq.modulation_freq += 0.1;
    } else if keyboard_input.just_pressed(KeyCode::Numpad1) {
        audio_freq.freq_diff -= 0.5;
    } else if keyboard_input.just_pressed(KeyCode::Numpad2) {
        audio_freq.freq_diff += 0.5;
    } else if keyboard_input.just_pressed(KeyCode::Numpad9) {
        audio_freq.audio_wave_type = match audio_freq.audio_wave_type {
            AudioWaveType::Sine => AudioWaveType::Squre,
            AudioWaveType::Squre => AudioWaveType::Saw,
            AudioWaveType::Saw => AudioWaveType::Sine,
        }
    } else if keyboard_input.just_pressed(KeyCode::Z) {
        println!(
            "Biip - Carrier: {}Hz Mod: {}Hz Diff(Variant): {}Hz",
            audio_freq.carrier_freq, audio_freq.modulation_freq, audio_freq.freq_diff
        );
        commands.spawn(AudioSourceBundle {
            source: assets.add(GeneratedAudio {
                carrier_frequency: audio_freq.carrier_freq,
                modulation_frequency: audio_freq.modulation_freq,
                frequency_diff: audio_freq.freq_diff,
                audio_wave_type: audio_freq.audio_wave_type,
            }),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}
