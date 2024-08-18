use crate::config::{Config, GameState, Position, ZLayer};
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;
use std::collections::HashSet;

pub struct FoodPlugin;

#[derive(Component)]
pub struct Food;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), init)
            .add_systems(Update, check_spawn.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Resource)]
struct FoodSpawnTimer(Timer);

fn check_spawn(
    time: Res<Time>,
    mut timer: ResMut<FoodSpawnTimer>,
    query: Query<&Position>,
    config: Res<Config>,
    commands: Commands,
) {
    if timer.0.tick(time.delta()).just_finished() {
        spawn(query, config, commands);
    }
}

fn init(query: Query<&Position>, config: Res<Config>, mut commands: Commands) {
    commands.insert_resource(FoodSpawnTimer(Timer::from_seconds(
        3.0,
        TimerMode::Repeating,
    )));
    spawn(query, config, commands);
}

fn spawn(query: Query<&Position>, config: Res<Config>, mut commands: Commands) {
    let mut blocked = HashSet::<Position>::new();
    for pos in query.iter() {
        blocked.insert(pos.clone());
    }
    let mut candidates = Vec::<Position>::new();
    for x in 0..config.grid_size_x {
        for y in 0..config.grid_size_y {
            let pos = Position { x, y };
            if !blocked.contains(&pos) {
                candidates.push(pos);
            }
        }
    }
    if candidates.is_empty() {
        panic!("No more space to spawn food!")
    }
    let spawn_pos = &candidates[thread_rng().gen_range(0..candidates.len())];

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::RegularPolygon {
                sides: 4,
                feature: shapes::RegularPolygonFeature::SideLength(
                    config.pixels_per_cell as f32 - 3.0,
                ),
                ..default()
            }),
            ..default()
        },
        Fill::color(css::SALMON),
        Food,
        spawn_pos.clone(),
        ZLayer { z: 2 },
    ));
}
