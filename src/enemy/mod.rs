use crate::{GameSet, AppState};
use bevy::prelude::*;
use systems::*;

pub mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_enemy_collision_system.after(GameSet::Movement),
                check_player_collision_system.after(GameSet::Movement),
                enemy_shoot_system,
            ).run_if(in_state(AppState::Gameplay)),
        );
    }
}
