use crate::enemy::components::{EnemyDestroyedEvent, EnemyShootEvent};
use crate::player::PlayerShootEvent;
use crate::utils::wave::{fade_out, normalized_sin, saw_wave, square_wave};
use crate::GameSet;
use bevy::audio::AddAudioSource;
use bevy::prelude::*;
use rand::Rng;

use crate::audio::generated_audio::GeneratedAudio;

mod generated_audio;
mod generated_audio_decoder;

const PLAYER_ATTACK_SOUND: GeneratedAudio = GeneratedAudio {
    carrier_frequency: 300.0,
    duration: 1.0,
    main_function: normalized_sin,
    volume_function: fade_out,
    frequency_function: |t| f32::max(1.0 - t, 0.3),
};

const ENEMY_ATTACK_SOUND: GeneratedAudio = GeneratedAudio {
    carrier_frequency: 100.0,
    duration: 0.3,
    main_function: square_wave,
    volume_function: |t| {
        if t < 0.1 {
            0.2 * t / 0.1
        } else if t > 0.7 {
            0.2 * (1.0 - ((t - 0.7) / 0.3))
        } else {
            0.2 * 1.0
        }
    },
    frequency_function: |_t| 1.0,
};

const ENEMY_DESTROYED_SOUND: GeneratedAudio = GeneratedAudio {
    carrier_frequency: 200.0,
    duration: 0.3,
    main_function: saw_wave,
    volume_function: |_t| 0.5,
    frequency_function: |t| 1.0 / f32::powi(t + 1.0, 4),
};

pub struct AudioPlayerPlugin;

impl Plugin for AudioPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<GeneratedAudio>()
            .insert_resource::<GeneratedAudio>(ENEMY_DESTROYED_SOUND)
            .add_systems(
                Update,
                (
                    sound_tester.in_set(GameSet::Input),
                    player_shoot_audio,
                    enemy_shoot_audio,
                    enemy_destroyed_audio,
                ),
            );
    }
}

fn enemy_destroyed_audio(
    mut commands: Commands,
    mut assets: ResMut<Assets<GeneratedAudio>>,
    mut ev_enemy_destroyed: EventReader<EnemyDestroyedEvent>,
) {
    for _ev in ev_enemy_destroyed.iter() {
        commands.spawn(AudioSourceBundle {
            source: assets.add(ENEMY_DESTROYED_SOUND),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}

fn player_shoot_audio(
    mut commands: Commands,
    mut assets: ResMut<Assets<GeneratedAudio>>,
    mut ev_player_shoot: EventReader<PlayerShootEvent>,
) {
    for _ev in ev_player_shoot.iter() {
        commands.spawn(AudioSourceBundle {
            source: assets.add(PLAYER_ATTACK_SOUND),
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
    let mut rng = rand::thread_rng();
    for _ev in ev_enemy_shoot.iter() {
        let mut sound_to_play = ENEMY_ATTACK_SOUND;
        sound_to_play.carrier_frequency = rng.gen_range(100.0..400.0);
        commands.spawn(AudioSourceBundle {
            source: assets.add(sound_to_play),
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
    mut audio_freq: ResMut<GeneratedAudio>,
) {
    if keyboard_input.just_pressed(KeyCode::Numpad7) {
        audio_freq.carrier_frequency -= 10.0;
    } else if keyboard_input.just_pressed(KeyCode::Numpad8) {
        audio_freq.carrier_frequency += 10.0;
    } else if keyboard_input.just_pressed(KeyCode::Z) {
        // 329.63 392.00 493.88
        println!("Biip - Frequency: {}Hz", audio_freq.carrier_frequency,);
        commands.spawn(AudioSourceBundle {
            source: assets.add(GeneratedAudio {
                carrier_frequency: 329.63,
                duration: audio_freq.duration,
                main_function: audio_freq.main_function,
                volume_function: audio_freq.volume_function,
                frequency_function: audio_freq.frequency_function,
            }),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
        });
        commands.spawn(AudioSourceBundle {
            source: assets.add(GeneratedAudio {
                carrier_frequency: 392.0,
                duration: audio_freq.duration,
                main_function: audio_freq.main_function,
                volume_function: audio_freq.volume_function,
                frequency_function: audio_freq.frequency_function,
            }),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
        });
        commands.spawn(AudioSourceBundle {
            source: assets.add(GeneratedAudio {
                carrier_frequency: 493.88,
                duration: audio_freq.duration,
                main_function: audio_freq.main_function,
                volume_function: audio_freq.volume_function,
                frequency_function: audio_freq.frequency_function,
            }),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}
