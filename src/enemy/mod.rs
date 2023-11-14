use crate::{AppState, GameSet};
use bevy::prelude::*;
use systems::*;

use self::components::EnemyShootEvent;

pub mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyShootEvent>()
            .add_systems(
                Update,
                (
                    check_enemy_collision_system.after(GameSet::Movement),
                    check_player_collision_system.after(GameSet::Movement),
                    enemy_shoot_system,
                )
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(OnEnter(AppState::Gameplay), reset_enemy);
    }
}
