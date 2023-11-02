use super::components::*;
use crate::bullet::Bullet;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub fn create_enemies_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    const X_INTERSPACE: f32 = 0.5 + 0.5;
    const X_OFFSET: f32 = -(X_INTERSPACE / 2.0 + X_INTERSPACE * 4.0);
    const Y_OFFSET: f32 = 4.0;

    for y in 0..4 {
        for x in 0..10 {
            commands.spawn(EnemyBundle::new(
                Vec2::new(
                    X_OFFSET + X_INTERSPACE * x as f32,
                    Y_OFFSET - y as f32 * X_INTERSPACE,
                ),
                &asset_server,
            ));
        }
    }
}

pub fn check_enemy_collision_system(
    mut commands: Commands,
    bullet_query: Query<(&Transform, &Sprite, Entity), With<Bullet>>,
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
