use crate::config::{Config, GameState, Position, ZLayer};
use bevy::color::palettes::css;
use bevy::prelude::*;
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

#[derive(Resource, Clone)]
struct FoodAssets {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

fn check_spawn(
    time: Res<Time>,
    mut timer: ResMut<FoodSpawnTimer>,
    query: Query<&Position>,
    config: Res<Config>,
    commands: Commands,
    assets: Res<FoodAssets>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        spawn(query, config, commands, &assets);
    }
}

fn init(
    query: Query<&Position>,
    config: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(FoodSpawnTimer(Timer::from_seconds(
        3.0,
        TimerMode::Repeating,
    )));
    let assets = FoodAssets {
        mesh: meshes.add(Rectangle::new(
            config.pixels_per_cell as f32 - 3.0,
            config.pixels_per_cell as f32 - 3.0,
        )),
        material: materials.add(Color::from(css::SALMON)),
    };
    commands.insert_resource(assets.clone());
    spawn(query, config, commands, &assets);
}

fn spawn(
    query: Query<&Position>,
    config: Res<Config>,
    mut commands: Commands,
    assets: &FoodAssets,
) {
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
    let spawn_pos = &candidates[rand::rng().random_range(0..candidates.len())];
    commands.spawn((
        Mesh2d(assets.mesh.clone()),
        MeshMaterial2d(assets.material.clone()),
        Food,
        spawn_pos.clone(),
        ZLayer { z: 2 },
    ));
}
