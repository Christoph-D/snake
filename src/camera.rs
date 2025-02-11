use crate::config::Config;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, fit_to_window);
    }
}

fn init(config: Res<Config>, mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            ((config.grid_size_x - 1) * config.pixels_per_cell) as f32 / 2.0,
            (config.grid_size_y * config.pixels_per_cell) as f32 / 2.0,
            0.0,
        ),
    ));
}

fn fit_to_window(
    config: Res<Config>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let window = window.single();
    let pixels_x = (config.grid_size_x * config.pixels_per_cell) as f32 + 50.0;
    let pixels_y = (config.grid_size_y * config.pixels_per_cell) as f32 + 50.0;
    let min_size = window.width().min(window.height());
    let mut transform = query.single_mut();
    transform.scale.x = pixels_x / min_size;
    transform.scale.y = pixels_y / min_size;
}
