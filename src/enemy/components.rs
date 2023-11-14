use crate::bullet::BulletShotCooldown;
use crate::movement::Velocity;
use bevy::prelude::*;
use std::f32::consts::PI;

// Determines how long enemies can wait between shots
// x is min cooldown
// y is max cooldown
pub const DEFAULT_BULLET_COOLDOWN: Vec2 = Vec2::new(2.0, 25.0);

#[derive(Event)]
pub struct EnemyShootEvent;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyBullet;

#[derive(Resource)]
pub struct EnemyBulletCooldown {
    pub min: f32,
    pub max: f32,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub sprite_bundle: SpriteBundle,
    pub velocity: Velocity,
    pub shot_cooldown: BulletShotCooldown,
    pub marker: Enemy,
}

impl EnemyBundle {
    pub fn new(position: Vec2, shot_cooldown: f32, asset_server: &Res<AssetServer>) -> Self {
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
            shot_cooldown: BulletShotCooldown(shot_cooldown),
            marker: Enemy,
        }
    }
}
