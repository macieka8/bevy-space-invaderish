use crate::bullet::{BulletBundle, BulletShotCooldown};
use crate::movement::Velocity;
use bevy::prelude::*;

use crate::player::*;

pub fn reset_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, With<Player>>,
) {
    for entity in &player_query {
        commands.entity(entity).despawn();
    }

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
}

pub fn player_movement_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let mut velocity = query.single_mut();
    let mut new_velocity = Velocity(Vec2::new(0.0, 0.0));
    if keyboard_input.pressed(MOVE_LEFT_KEYCODE) {
        new_velocity.x -= 2.5;
    }

    if keyboard_input.pressed(MOVE_RIGHT_KEYCODE) {
        new_velocity.x += 2.5;
    }

    *velocity = new_velocity;
}

pub fn player_shoot_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut BulletShotCooldown), With<Player>>,
) {
    let (player_transform, mut player_shot_cooldown) = player_query.single_mut();
    if player_shot_cooldown.0 <= 0.0 && keyboard_input.pressed(SHOOT_KEYCODE) {
        player_shot_cooldown.0 = PLAYER_SHOT_RATE;
        let missle_position = player_transform.translation + Vec3::Y * 0.7;
        commands.spawn(BulletBundle::new(
            missle_position.truncate(),
            Vec2::new(0.0, 2.5),
        ));
    }
}

pub fn player_movement_restriction_system(
    mut player_transform: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player_transform.single_mut();
    let position = transform.translation;
    if position.x > 5.5 {
        transform.translation = Vec3::new(5.5, position.y, position.z);
    } else if position.x < -5.5 {
        transform.translation = Vec3::new(-5.5, position.y, position.z);
    }
}
