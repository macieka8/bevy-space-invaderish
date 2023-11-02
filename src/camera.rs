use std::cmp::min;

use bevy::{
    prelude::*,
    render::camera::{ScalingMode, Viewport},
    window::WindowResized,
};

use crate::CAMERA_SIZE;

pub struct CenterCameraPlugin;

impl Plugin for CenterCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, set_camera_viewport);
    }
}

fn setup_camera(mut commands: Commands) {
    // spawn camera
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: CAMERA_SIZE.x,
        height: CAMERA_SIZE.y,
    };
    commands.spawn(camera);
}

fn set_camera_viewport(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut camera: Query<&mut Camera>,
) {
    for resize_event in resize_events.iter() {
        let window = windows.get(resize_event.window).unwrap();
        let mut camera = camera.single_mut();

        // This value make sure viewport size fits on the window's screen.
        let mult = min(
            window.resolution.physical_height() / CAMERA_SIZE.y as u32,
            window.resolution.physical_width() / CAMERA_SIZE.x as u32,
        );
        let viewport_size = UVec2 {
            x: mult * CAMERA_SIZE.x as u32,
            y: mult * CAMERA_SIZE.y as u32,
        };

        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(
                (window.resolution.physical_width() - viewport_size.x) / 2,
                (window.resolution.physical_height() - viewport_size.y) / 2,
            ),
            physical_size: viewport_size,
            ..default()
        });
    }
}
