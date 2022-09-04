use crate::config::{Config, Name};
use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

#[derive(Component)]
pub struct Grid;

#[derive(Bundle)]
pub struct GridBundle {
    pub grid: Grid,
    pub name: Name,
    #[bundle]
    pub shape_bundle: ShapeBundle,
}

impl GridBundle {
    pub fn from_config(config: &Config) -> GridBundle {
        let mut grid_builder = GeometryBuilder::new();
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
        GridBundle {
            grid: Grid,
            name: Name("grid".to_owned()),
            shape_bundle: grid_builder.build(
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 1.0)),
                Transform::default(),
            ),
        }
    }
}
