use crate::bullet::{BulletBundle, BulletShotCooldown};
use crate::collider::Collider;
use crate::enemy::components::EnemyBullet;
use crate::movement::Velocity;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::player::*;

pub fn reset_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(Entity, &Children), With<Player>>,
) {
    for (player, children) in &player_query {
        for &child in children.iter() {
            commands.entity(child).despawn();
        }
        commands.entity(player).despawn();
    }

    let player = PlayerBundle {
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
    };

    commands.spawn(player).with_children(|parent| {
        // Vertical collider
        parent.spawn((
            Collider {
                size: Vec2::new(0.5, 0.8),
            },
            TransformBundle {
                local: Transform::from_translation(Vec3::new(-0.05, 0.0, 0.0)),
                ..default()
            },
        ));

        // Bottom collider
        parent.spawn((
            Collider {
                size: Vec2::new(0.85, 0.25),
            },
            TransformBundle {
                local: Transform::from_translation(Vec3::new(0.0, -0.20, 0.0)),
                ..default()
            },
        ));
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
    mut ev_player_shoot: EventWriter<PlayerShootEvent>,
) {
    let (player_transform, mut player_shot_cooldown) = player_query.single_mut();
    if player_shot_cooldown.0 <= 0.0 && keyboard_input.pressed(SHOOT_KEYCODE) {
        player_shot_cooldown.0 = PLAYER_SHOT_RATE;
        let missle_position = player_transform.translation + Vec3::Y * 0.7;
        commands.spawn(BulletBundle::new(
            missle_position.truncate(),
            Vec2::new(0.0, 2.5),
        ));
        ev_player_shoot.send(PlayerShootEvent);
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

pub fn check_player_collision_system(
    mut commands: Commands,
    bullet_query: Query<(&Transform, &Sprite, Entity), With<EnemyBullet>>,
    player_query: Query<&Children, With<Player>>,
    collider_query: Query<(&GlobalTransform, &Collider)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (bullet_transform, bullet_sprite, bullet_entity) in &bullet_query {
        for children in &player_query {
            let bullet_size = bullet_sprite.custom_size.unwrap_or(Vec2::new(1.0, 1.0));
            // Iterate over all children with collider component
            for &child in children {
                let (global_transform, player_collider) = collider_query.get(child).unwrap();
                let collider_position = global_transform.translation();

                let collision = collide(
                    bullet_transform.translation,
                    bullet_size,
                    collider_position,
                    player_collider.size,
                );

                if collision.is_some() {
                    commands.entity(bullet_entity).despawn();
                    // todo: handle player got hit
                    next_state.set(AppState::Paused);
                }
            }
        }
    }
}
