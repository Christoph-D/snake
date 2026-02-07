use crate::config::{BACKGROUND_COLOR, GameState};
use bevy::color::palettes::css;
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
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        padding: UiRect::px(30.0, 30.0, 15.0, 30.0),
                        ..default()
                    },
                    BackgroundColor(BACKGROUND_COLOR),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((Text::new(""), TextLayout::new_with_justify(Justify::Center)))
                        .with_children(|parent| {
                            parent.spawn((
                                TextSpan::new("Game over!"),
                                TextFont {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 100.0,
                                    ..default()
                                },
                                TextColor(css::RED.into()),
                            ));
                            parent.spawn((
                                TextSpan::new("\nPress any key to restart"),
                                TextFont {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    ..default()
                                },
                                TextColor(css::RED.into()),
                            ));
                        });
                });
        });
}

fn read_restart_input(
    time: Res<Time>,
    mut timer: ResMut<GameOverWaitTimer>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    if keys.get_just_pressed().next().is_some() {
        next_state.set(GameState::InGame);
    }
}
