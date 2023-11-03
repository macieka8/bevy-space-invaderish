use bevy::prelude::*;
use rand::Rng;

use crate::{
    enemy::components::{Enemy, EnemyBundle, ENEMY_BULLET_SHOT_RATE},
    AppState,
};

#[derive(Resource)]
pub struct CurrentLevel(pub u32);

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevel(0))
            .add_systems(Update, level_loader.run_if(in_state(AppState::Gameplay)));
    }
}

pub fn level_loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_level: ResMut<CurrentLevel>,
    enemy_query: Query<(), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        current_level.0 += 1;
        match current_level.0 {
            1 => level_1(commands, asset_server),
            2 => level_2(commands, asset_server),
            3 => level_3(commands, asset_server),
            _ => println!("No more levels."),
        }
    }
}

pub fn level_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    let x_interspace: f32 = 0.5 + 3.0;
    let x_offset: f32 = -2.0;
    let y_offset: f32 = 4.0;

    let mut rng = rand::thread_rng();

    for y in 0..2 {
        for x in 0..2 {
            commands.spawn(EnemyBundle::new(
                Vec2::new(
                    x_offset + x_interspace * x as f32,
                    y_offset - y as f32 * x_interspace,
                ),
                rng.gen_range(ENEMY_BULLET_SHOT_RATE.x..ENEMY_BULLET_SHOT_RATE.y),
                &asset_server,
            ));
        }
    }
}

pub fn level_2(mut commands: Commands, asset_server: Res<AssetServer>) {
    let x_interspace: f32 = 0.5 + 0.5;
    let x_offset: f32 = -1.0;
    let y_offset: f32 = 4.0;

    let mut rng = rand::thread_rng();

    for y in 0..5 {
        for x in 0..3 {
            commands.spawn(EnemyBundle::new(
                Vec2::new(
                    x_offset + x_interspace * x as f32,
                    y_offset - y as f32 * x_interspace,
                ),
                rng.gen_range(ENEMY_BULLET_SHOT_RATE.x..ENEMY_BULLET_SHOT_RATE.y),
                &asset_server,
            ));
        }
    }
}

pub fn level_3(mut commands: Commands, asset_server: Res<AssetServer>) {
    let x_interspace: f32 = 0.5 + 0.5;
    let x_offset: f32 = -(x_interspace / 2.0 + x_interspace * 4.0);
    let y_offset: f32 = 4.0;

    let mut rng = rand::thread_rng();

    for y in 0..4 {
        for x in 0..10 {
            commands.spawn(EnemyBundle::new(
                Vec2::new(
                    x_offset + x_interspace * x as f32,
                    y_offset - y as f32 * x_interspace,
                ),
                rng.gen_range(ENEMY_BULLET_SHOT_RATE.x..ENEMY_BULLET_SHOT_RATE.y),
                &asset_server,
            ));
        }
    }
}
