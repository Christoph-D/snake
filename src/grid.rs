use crate::config::{Config, GameState};
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), init);
    }
}

fn create_grid_mesh(config: &Config) -> Mesh {
    let xmax = (config.grid_size_x * config.pixels_per_cell) as f32;
    let ymax = (config.grid_size_y * config.pixels_per_cell) as f32;
    let half_cell = config.pixels_per_cell as f32 / 2.0;
    let line_width = 1.0; // Width of the grid lines
    let w = line_width / 2.0;

    let mut positions = Vec::new();
    let mut indices = Vec::new();

    // Add vertical lines
    for i in 0..config.grid_size_x + 1 {
        let x = (i * config.pixels_per_cell) as f32 - half_cell;
        let y_start = -half_cell;
        let y_end = ymax - half_cell;

        // Create a rectangle for each line segment
        let start_index = positions.len() as u32;
        positions.push([x - w, y_start, 0.0]); // Bottom-left
        positions.push([x + w, y_start, 0.0]); // Bottom-right
        positions.push([x - w, y_end, 0.0]); // Top-left
        positions.push([x + w, y_end, 0.0]); // Top-right

        // Two triangles to form a rectangle
        indices.push(start_index);
        indices.push(start_index + 1);
        indices.push(start_index + 2);

        indices.push(start_index + 2);
        indices.push(start_index + 1);
        indices.push(start_index + 3);
    }

    // Add horizontal lines
    for i in 0..config.grid_size_y + 1 {
        let y = (i * config.pixels_per_cell) as f32 - half_cell;
        let x_start = -half_cell;
        let x_end = xmax - half_cell;

        // Create a rectangle for each line segment
        let start_index = positions.len() as u32;
        positions.push([x_start, y - w, 0.0]); // Bottom-left
        positions.push([x_start, y + w, 0.0]); // Top-left
        positions.push([x_end, y - w, 0.0]); // Bottom-right
        positions.push([x_end, y + w, 0.0]); // Top-right

        // Two triangles to form a rectangle
        indices.push(start_index);
        indices.push(start_index + 1);
        indices.push(start_index + 2);

        indices.push(start_index + 2);
        indices.push(start_index + 1);
        indices.push(start_index + 3);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![[1.0, 1.0, 1.0, 1.0]; positions.len()],
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

fn init(
    config: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grid_mesh = create_grid_mesh(&config);
    let mesh_handle = meshes.add(grid_mesh);
    let material_handle = materials.add(ColorMaterial::from(Color::WHITE));

    commands.spawn((Mesh2d(mesh_handle), MeshMaterial2d(material_handle)));
}
