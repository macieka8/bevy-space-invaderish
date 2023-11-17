use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_colliders_gizmo);
    }
}

fn draw_colliders_gizmo(mut gizmos: Gizmos, query: Query<(&GlobalTransform, &Collider)>) {
    for (transform, collider) in &query {
        gizmos.rect_2d(
            transform.translation().truncate(),
            0.0,
            collider.size,
            Color::PINK,
        );
    }
}
