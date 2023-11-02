use crate::movement::Velocity;
use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

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

pub fn destroy_faraway_bullets_system(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<Bullet>>,
) {
    for (transform, entity) in query.iter() {
        if transform.translation.y > 6.0 {
            commands.entity(entity).despawn();
        }
    }
}
