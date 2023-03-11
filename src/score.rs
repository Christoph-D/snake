use crate::config::GameState;
use bevy::prelude::*;

pub struct ScorePlugin;

pub enum ScoreUpdate {
    AteFood,
}

#[derive(Resource)]
struct ScoreValue(i32);

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreUpdate>()
            .add_system(init.in_schedule(OnEnter(GameState::InGame)))
            .add_system(update.in_set(OnUpdate(GameState::InGame)));
    }
}

#[derive(Component)]
struct Score;

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ScoreValue(0));
    commands.spawn((
        TextBundle::from_section(
            "", // Updated later
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(50.0),
                top: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        Score,
    ));
}

fn update(
    mut score: ResMut<ScoreValue>,
    mut query: Query<&mut Text, With<Score>>,
    mut event: EventReader<ScoreUpdate>,
) {
    let mut text = query.single_mut();
    for _ in event.iter() {
        score.0 += 1;
    }
    text.sections[0].value = format!("Score: {}", score.0);
}
