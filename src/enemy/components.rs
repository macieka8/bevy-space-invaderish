use crate::movement::Velocity;
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub sprite_bundle: SpriteBundle,
    pub velocity: Velocity,
    pub marker: Enemy,
}

impl EnemyBundle {
    pub fn new(position: Vec2, asset_server: &Res<AssetServer>) -> Self {
        let mut transform = Transform::from_xyz(position.x, position.y, 4.0);
        transform.rotate_local_z(PI);
        EnemyBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("images/ship.png"),
                transform,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.5, 0.5)),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity(Vec2::ZERO),
            marker: Enemy,
        }
    }
}
