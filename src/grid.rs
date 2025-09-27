use crate::config::{Config, GameState};
use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), init);
    }
}

fn init(config: Res<Config>, mut commands: Commands) {
    commands.spawn(Grid::from_config(&config));
}

#[derive(Bundle)]
struct Grid(Shape);

impl Grid {
    fn from_config(config: &Config) -> Grid {
        let mut grid_builder = ShapeBuilder::new();
        let xmax = (config.grid_size_x * config.pixels_per_cell) as f32;
        let ymax = (config.grid_size_y * config.pixels_per_cell) as f32;
        let half_cell = config.pixels_per_cell as f32 / 2.0;
        for i in 0..config.grid_size_x + 1 {
            let x = (i * config.pixels_per_cell) as f32;
            grid_builder = grid_builder.add(&shapes::Line(
                Vec2 {
                    x: x - half_cell,
                    y: -half_cell,
                },
                Vec2 {
                    x: x - half_cell,
                    y: ymax - half_cell,
                },
            ));
        }
        for i in 0..config.grid_size_y + 1 {
            let y = (i * config.pixels_per_cell) as f32;
            grid_builder = grid_builder.add(&shapes::Line(
                Vec2 {
                    x: -half_cell,
                    y: y - half_cell,
                },
                Vec2 {
                    x: xmax - half_cell,
                    y: y - half_cell,
                },
            ));
        }
        Grid(grid_builder.stroke((Color::WHITE, 1.0)).build())
    }
}
