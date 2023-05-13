use crate::config::*;
use bevy::prelude::*;
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
enum BaseSet {
    UpdateTransformations,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".to_owned(),
                canvas: Some("#game".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_plugin(ShapePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(game_over::GameOverScreenPlugin)
        .add_plugin(grid::GridPlugin)
        .add_plugin(score::ScorePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(food::FoodPlugin)
        .insert_resource(Config {
            grid_size_x: 20,
            grid_size_y: 20,
            pixels_per_cell: 30,
        })
        .add_system(bevy::window::close_on_esc)
        .add_system(despawn_all.in_schedule(OnEnter(GameState::InGame)))
        .add_system(update_transformations.in_base_set(BaseSet::UpdateTransformations))
        .configure_set(
            BaseSet::UpdateTransformations
                .after(CoreSet::Update)
                .before(CoreSet::PostUpdate),
        )
        .run();
}
