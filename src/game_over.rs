use crate::config::{GameState, BACKGROUND_COLOR};
use bevy::prelude::*;

pub struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), show_game_over_screen)
            .add_systems(
                Update,
                read_restart_input.run_if(in_state(GameState::GameOver)),
            );
    }
}

#[derive(Resource)]
struct GameOverWaitTimer(Timer);

fn show_game_over_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameOverWaitTimer(Timer::from_seconds(0.2, TimerMode::Once)));
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        align_self: AlignSelf::Center,
                        padding: UiRect::all(Val::Px(30.0)),
                        ..default()
                    },
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_sections(vec![
                            TextSection {
                                value: "Game over!".to_owned(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 100.0,
                                    color: Color::RED,
                                },
                            },
                            TextSection {
                                value: "\nPress any key to restart".to_owned(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::RED,
                                },
                            },
                        ])
                        .with_text_justify(JustifyText::Center)
                        .with_style(Style {
                            align_self: AlignSelf::Center,
                            ..default()
                        }),
                    );
                });
        });
}

fn read_restart_input(
    time: Res<Time>,
    mut timer: ResMut<GameOverWaitTimer>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    if keys.get_just_pressed().next().is_some() {
        next_state.set(GameState::InGame);
    }
}
