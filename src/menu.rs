use bevy::prelude::*;

use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Paused), enter_menu)
            .add_systems(OnExit(AppState::Paused), exit_menu)
            .add_systems(Update, play_again_system.run_if(in_state(AppState::Paused)));
    }
}

fn play_again_system(
    interation_query: Query<&Interaction, (With<Button>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        next_state.set(AppState::Gameplay);
    }
    for interaction in &interation_query {
        match interaction {
            Interaction::Pressed => {
                println!("Pressed");
                next_state.set(AppState::Gameplay);
            }
            Interaction::Hovered => println!("Hovered"),
            Interaction::None => (),
        }
    }
}

fn exit_menu(mut commands: Commands, mut query: Query<Entity, With<Node>>) {
    for entity in &mut query {
        commands.entity(entity).despawn();
    }
}

fn enter_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::ANTIQUE_WHITE),
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play again",
                        TextStyle {
                            font: asset_server.load("fonts/Ubuntu-Regular.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
