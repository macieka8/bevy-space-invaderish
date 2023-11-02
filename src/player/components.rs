use crate::movement::Velocity;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite_bundle: SpriteBundle,
    pub velocity: Velocity,
    pub marker: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            marker: Player,
        }
    }
}
