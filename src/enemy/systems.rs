use super::components::*;
use crate::bullet::{Bullet, BulletBundle, BulletShotCooldown};
use crate::player::Player;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

pub fn check_enemy_collision_system(
    mut commands: Commands,
    bullet_query: Query<(&Transform, &Sprite, Entity), (With<Bullet>, Without<EnemyBullet>)>,
    enemy_query: Query<(&Transform, &Sprite, Entity), With<Enemy>>,
) {
    for (bullet_transform, bullet_sprite, bullet_entity) in &bullet_query {
        for (enemy_transform, enemy_sprite, enemy_entity) in &enemy_query {
            // TODO: Handle changed scale
            let bullet_size = bullet_sprite.custom_size.unwrap_or(Vec2::new(1.0, 1.0));
            let enemy_size = enemy_sprite.custom_size.unwrap_or(Vec2::new(1.0, 1.0));
            let collision = collide(
                bullet_transform.translation,
                bullet_size,
                enemy_transform.translation,
                enemy_size,
            );

            if collision.is_some() {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}

pub fn check_player_collision_system(
    mut commands: Commands,
    bullet_query: Query<(&Transform, &Sprite, Entity), With<EnemyBullet>>,
    player_query: Query<(&Transform, &Sprite, Entity), With<Player>>,
) {
    for (bullet_transform, bullet_sprite, bullet_entity) in &bullet_query {
        for (player_transform, player_sprite, player_entity) in &player_query {
            // TODO: Handle changed scale
            let bullet_size = bullet_sprite.custom_size.unwrap_or(Vec2::new(1.0, 1.0));
            let player_size = player_sprite.custom_size.unwrap_or(Vec2::new(1.0, 1.0));
            let collision = collide(
                bullet_transform.translation,
                bullet_size,
                player_transform.translation,
                player_size,
            );

            if collision.is_some() {
                commands.entity(bullet_entity).despawn();
                // todo: handle player got hit
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn enemy_shoot_system(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(&mut BulletShotCooldown, &Transform), With<Enemy>>,
) {
    let mut rng = rand::thread_rng();

    for (mut bullet_shot_cooldown, transform) in enemy_query.iter_mut() {
        bullet_shot_cooldown.0 -= time.delta().as_secs_f32();
        if bullet_shot_cooldown.0 <= 0.0 {
            bullet_shot_cooldown.0 =
                rng.gen_range(ENEMY_BULLET_SHOT_RATE.x..ENEMY_BULLET_SHOT_RATE.y);

            let bullet_position = Vec2::new(transform.translation.x, transform.translation.y - 0.4);
            commands.spawn((
                BulletBundle::new(bullet_position, Vec2::new(0.0, -2.5)),
                EnemyBullet,
            ));
        }
    }
}
