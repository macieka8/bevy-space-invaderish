use crate::bullet::BulletBundle;
use crate::movement::Velocity;
use bevy::prelude::*;

use crate::player::*;

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
    player_query: Query<&Transform, With<Player>>,
) {
    if keyboard_input.just_pressed(SHOOT_KEYCODE) {
        let player_transform = player_query.single();
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
