use crate::config::{GameState, BACKGROUND_COLOR};
use bevy::prelude::*;

pub struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(show_game_over_screen),
        )
        .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(read_restart_input));
    }
}

fn show_game_over_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        align_self: AlignSelf::Center,
                        padding: UiRect::all(Val::Px(30.0)),
                        ..Default::default()
                    },
                    color: BACKGROUND_COLOR.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(
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
                        .with_text_alignment(TextAlignment::TOP_CENTER)
                        .with_style(Style {
                            align_self: AlignSelf::Center,
                            ..default()
                        }),
                    );
                });
        });
}

fn read_restart_input(keys: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keys.get_just_pressed().next().is_some() {
        game_state.set(GameState::InGame).unwrap();
    }
}
