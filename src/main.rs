use bevy::{prelude::*, window::WindowResolution};
use bullet::destroy_faraway_bullets_system;
use camera::CenterCameraPlugin;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use player::{PlayerBundle, PlayerPlugin};

mod bullet;
mod camera;
mod enemy;
mod movement;
mod player;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}

const CAMERA_SIZE: Vec2 = Vec2::new(12.0, 12.0);

const WINDOW_RESOLUTION_WIDTH: f32 = 800.0;
const WINDOW_RESOLUTION_HEIGHT: f32 = 600.0;

fn main() {
    App::new()
        .configure_set(Update, GameSet::Input)
        .configure_set(FixedUpdate, GameSet::Movement)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
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
            }),
            PlayerPlugin,
            CenterCameraPlugin,
            MovementPlugin,
            EnemyPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, destroy_faraway_bullets_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn player
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("images/ship.png"),
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            ..default()
        },
        ..default()
    });

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
