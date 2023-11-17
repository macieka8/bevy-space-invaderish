use crate::{AppState, GameSet};
use bevy::prelude::*;
pub use components::*;
use systems::*;

pub mod components;
mod systems;

pub const SHOOT_KEYCODE: KeyCode = KeyCode::J;
pub const MOVE_LEFT_KEYCODE: KeyCode = KeyCode::A;
pub const MOVE_RIGHT_KEYCODE: KeyCode = KeyCode::D;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerShootEvent>()
            .add_systems(
                FixedUpdate,
                player_movement_restriction_system
                    .after(GameSet::Movement)
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(
                Update,
                (player_shoot_input_system, player_movement_input_system)
                    .in_set(GameSet::Input)
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(OnEnter(AppState::Gameplay), reset_player)
            .add_systems(
                Update,
                check_player_collision_system
                    .after(GameSet::Movement)
                    .run_if(in_state(AppState::Gameplay)),
            );
    }
}
