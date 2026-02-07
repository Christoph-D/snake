use crate::config::GameState;
use bevy::prelude::*;

pub struct ScorePlugin;

#[derive(Message)]
pub enum ScoreUpdate {
    AteFood,
}

#[derive(Resource)]
struct ScoreValue(i32);

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ScoreUpdate>()
            .add_systems(OnEnter(GameState::InGame), init)
            .add_systems(Update, update);
    }
}

#[derive(Component)]
struct Score;

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ScoreValue(0));
    commands.spawn((
        Text::new(""), // Updated later
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Left),
        Node {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            left: Val::Px(50.0),
            top: Val::Px(5.0),
            ..default()
        },
        Score,
    ));
}

fn update(
    mut score: ResMut<ScoreValue>,
    mut query: Query<&mut Text, With<Score>>,
    mut event: MessageReader<ScoreUpdate>,
) {
    let mut text = query.single_mut().unwrap();
    for _ in event.read() {
        score.0 += 1;
    }
    text.0 = format!("Score: {}", score.0);
}
