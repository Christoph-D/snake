use crate::config::{Config, GameState, Position, ZLayer};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;
use std::collections::HashSet;

pub struct FoodPlugin;

#[derive(Component)]
pub struct Food;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(init))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(check_spawn));
    }
}

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
    commands.insert_resource(FoodSpawnTimer(Timer::from_seconds(3.0, true)));
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

    commands
        .spawn()
        .insert(Food)
        .insert(spawn_pos.clone())
        .insert(ZLayer { z: 2 })
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::RegularPolygon {
                sides: 4,
                feature: shapes::RegularPolygonFeature::SideLength(
                    config.pixels_per_cell as f32 - 3.0,
                ),
                ..shapes::RegularPolygon::default()
            },
            DrawMode::Fill(FillMode::color(Color::SALMON)),
            Transform::default(),
        ));
}
