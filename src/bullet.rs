use crate::{levels::LevelChangedEvent, movement::Velocity, AppState};
use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletShotCooldown(pub f32);

#[derive(Bundle)]
pub struct BulletBundle {
    pub sprite_bundle: SpriteBundle,
    pub velocity: Velocity,
    pub marker: Bullet,
}

impl BulletBundle {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        BulletBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 5.0)),
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(0.1, 0.2)),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity(velocity),
            marker: Bullet,
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Gameplay), destroy_all_bullets)
            .add_systems(
                Update,
                (
                    destroy_faraway_bullets_system,
                    on_level_changed_event,
                    update_bullet_shot_cooldown,
                )
                    .run_if(in_state(AppState::Gameplay)),
            );
    }
}

fn on_level_changed_event(
    commands: Commands,
    query: Query<Entity, With<Bullet>>,
    mut ev_level_changed: EventReader<LevelChangedEvent>,
) {
    if !ev_level_changed.is_empty() {
        ev_level_changed.clear();
        destroy_all_bullets(commands, query);
    }
}

fn destroy_faraway_bullets_system(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<Bullet>>,
) {
    for (transform, entity) in query.iter() {
        if transform.translation.y > 6.0 || transform.translation.y < -6.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn destroy_all_bullets(mut commands: Commands, query: Query<Entity, With<Bullet>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn update_bullet_shot_cooldown(
    time: Res<Time>,
    mut cooldown_query: Query<&mut BulletShotCooldown>,
) {
    for mut bullet_shot_cooldown in &mut cooldown_query {
        if bullet_shot_cooldown.0 > 0.0 {
            bullet_shot_cooldown.0 -= time.delta_seconds();
        }
    }
}
