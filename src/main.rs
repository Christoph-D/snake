use crate::config::*;
use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use bevy_prototype_lyon::prelude::*;

mod camera;
mod config;
mod food;
mod game_over;
mod grid;
mod player;
mod score;

fn despawn_all(
    all_except_necessary: Query<Entity, (Without<Camera2d>, Without<Window>)>,
    mut commands: Commands,
) {
    for entity in all_except_necessary.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_transformations(
    config: Res<Config>,
    mut query: Query<(&mut Transform, &Position, &ZLayer)>,
) {
    for (mut transform, pos, z_layer) in query.iter_mut() {
        *transform = Transform::from_xyz(
            (config.pixels_per_cell * pos.x) as f32,
            (config.pixels_per_cell * pos.y) as f32,
            z_layer.z as f32,
        )
    }
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct UpdateTransformations;

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }
        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".to_owned(),
                canvas: Some("#game".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_plugins((
            ShapePlugin,
            camera::CameraPlugin,
            game_over::GameOverScreenPlugin,
            grid::GridPlugin,
            score::ScorePlugin,
            player::PlayerPlugin,
            food::FoodPlugin,
        ))
        .insert_resource(Config {
            grid_size_x: 20,
            grid_size_y: 20,
            pixels_per_cell: 30,
        })
        .add_systems(Update, close_on_esc)
        .add_systems(OnExit(GameState::GameOver), despawn_all)
        .init_schedule(UpdateTransformations)
        .add_systems(UpdateTransformations, update_transformations);

    let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
    order.insert_after(Update, UpdateTransformations);

    app.run();
}
