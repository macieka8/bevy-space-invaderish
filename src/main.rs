use audio::AudioPlayerPlugin;
use bevy::{
    audio::{AudioPlugin, VolumeLevel},
    prelude::*,
    window::WindowResolution,
};
use bullet::BulletPlugin;
use camera::CenterCameraPlugin;
use collider::ColliderPlugin;
use enemy::EnemyPlugin;
use levels::LevelsPlugin;
use menu::MenuPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

mod audio;
mod bullet;
mod camera;
mod collider;
mod enemy;
mod levels;
mod menu;
mod movement;
mod player;
mod utils;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    Gameplay,
    Paused,
}

const CAMERA_SIZE: Vec2 = Vec2::new(12.0, 12.0);

const WINDOW_RESOLUTION_WIDTH: f32 = 800.0;
const WINDOW_RESOLUTION_HEIGHT: f32 = 600.0;

fn main() {
    App::new()
        .add_state::<AppState>()
        .configure_set(Update, GameSet::Input)
        .configure_set(FixedUpdate, GameSet::Movement)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(
                            WINDOW_RESOLUTION_WIDTH,
                            WINDOW_RESOLUTION_HEIGHT,
                        ),
                        title: String::from("Space Invader in Bevy"),
                        ..default()
                    }),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: VolumeLevel::new(0.1),
                    },
                }),
            PlayerPlugin,
            CenterCameraPlugin,
            MovementPlugin,
            EnemyPlugin,
            LevelsPlugin,
            MenuPlugin,
            BulletPlugin,
            AudioPlayerPlugin,
            ColliderPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn background image
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/space.png"),
        sprite: Sprite {
            custom_size: Some(CAMERA_SIZE),
            ..default()
        },
        ..default()
    });
}
