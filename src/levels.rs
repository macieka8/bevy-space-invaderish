use bevy::prelude::*;
use rand::Rng;

use crate::{
    enemy::components::{Enemy, EnemyBulletCooldown, EnemyBundle, DEFAULT_BULLET_COOLDOWN},
    AppState,
};

#[derive(Resource)]
pub struct CurrentLevel(pub u32);

#[derive(Event)]
pub struct LevelChangedEvent;

pub struct LevelsPlugin;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LevelSet {
    LevelLoader,
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevel(0))
            .insert_resource(EnemyBulletCooldown {
                min: DEFAULT_BULLET_COOLDOWN.x,
                max: DEFAULT_BULLET_COOLDOWN.y,
            })
            .configure_set(Update, LevelSet::LevelLoader)
            .add_event::<LevelChangedEvent>()
            .add_systems(
                Update,
                level_loader
                    .in_set(LevelSet::LevelLoader)
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(OnEnter(AppState::Gameplay), reset_level);
    }
}

fn reset_level(
    mut current_level: ResMut<CurrentLevel>,
    mut enemy_bullet_cooldown: ResMut<EnemyBulletCooldown>,
) {
    current_level.0 = 0;
    *enemy_bullet_cooldown = EnemyBulletCooldown {
        min: DEFAULT_BULLET_COOLDOWN.x,
        max: DEFAULT_BULLET_COOLDOWN.y,
    }
}

fn level_loader(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_level: ResMut<CurrentLevel>,
    enemy_bullet_cooldown: ResMut<EnemyBulletCooldown>,
    enemy_query: Query<(), With<Enemy>>,
    mut ev_level_changed: EventWriter<LevelChangedEvent>,
) {
    if enemy_query.is_empty() {
        current_level.0 += 1;
        match current_level.0 {
            1 => level_1(commands, asset_server, enemy_bullet_cooldown, current_level),
            2 => level_2(commands, asset_server, enemy_bullet_cooldown, current_level),
            _ => next_levels(commands, asset_server, enemy_bullet_cooldown, current_level),
        }

        ev_level_changed.send(LevelChangedEvent);
    }
}

fn level_1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_bullet_cooldown: ResMut<EnemyBulletCooldown>,
    current_level: ResMut<CurrentLevel>,
) {
    *enemy_bullet_cooldown = EnemyBulletCooldown {
        min: DEFAULT_BULLET_COOLDOWN.x / 2.0,
        max: DEFAULT_BULLET_COOLDOWN.y / 2.0,
    };

    println!(
        "Level {:?}, Enemy Bullet Cd: ({:?}, {:?})",
        current_level.0, enemy_bullet_cooldown.min, enemy_bullet_cooldown.max
    );

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
                rng.gen_range(enemy_bullet_cooldown.min..=enemy_bullet_cooldown.max),
                &asset_server,
            ));
        }
    }
}

fn level_2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_bullet_cooldown: ResMut<EnemyBulletCooldown>,
    current_level: ResMut<CurrentLevel>,
) {
    *enemy_bullet_cooldown = EnemyBulletCooldown {
        min: DEFAULT_BULLET_COOLDOWN.x / 3.0,
        max: DEFAULT_BULLET_COOLDOWN.y / 3.0,
    };

    println!(
        "Level {:?}, Enemy Bullet Cd: ({:?}, {:?})",
        current_level.0, enemy_bullet_cooldown.min, enemy_bullet_cooldown.max
    );

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
                rng.gen_range(enemy_bullet_cooldown.min..=enemy_bullet_cooldown.max),
                &asset_server,
            ));
        }
    }
}

fn next_levels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_bullet_cooldown: ResMut<EnemyBulletCooldown>,
    current_level: ResMut<CurrentLevel>,
) {
    *enemy_bullet_cooldown = EnemyBulletCooldown {
        min: DEFAULT_BULLET_COOLDOWN.x / current_level.0 as f32,
        max: DEFAULT_BULLET_COOLDOWN.y / current_level.0 as f32,
    };

    println!(
        "Level {:?}, Enemy Bullet Cd: ({:?}, {:?})",
        current_level.0, enemy_bullet_cooldown.min, enemy_bullet_cooldown.max
    );

    let x_interspace: f32 = 0.5 + 0.5;
    let x_offset: f32 = -(x_interspace / 2.0 + x_interspace * 4.0);
    let y_offset: f32 = 4.0;

    let mut rng = rand::thread_rng();

    let row_count = 1 + current_level.0 / 5;
    for y in 0..row_count {
        for x in 0..10 {
            commands.spawn(EnemyBundle::new(
                Vec2::new(
                    x_offset + x_interspace * x as f32,
                    y_offset - y as f32 * x_interspace,
                ),
                rng.gen_range(enemy_bullet_cooldown.min..=enemy_bullet_cooldown.max),
                &asset_server,
            ));
        }
    }
}
