use bevy::prelude::*;

use crate::GameSet;

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Velocity(pub Vec2);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_movement_system.in_set(GameSet::Movement),
        );
    }
}

pub fn update_movement_system(
    mut query: Query<(&mut Transform, &Velocity)>,
    time_step: Res<FixedTime>,
) {
    let delta_time = time_step.period.as_secs_f32();
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * delta_time;
        transform.translation.y += velocity.y * delta_time;
    }
}
