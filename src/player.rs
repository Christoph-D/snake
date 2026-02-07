use crate::config::*;
use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), init)
            .add_systems(
                Update,
                (
                    read_player_input.before(apply_player_input),
                    apply_player_input.before(move_player),
                    move_player
                        .before(check_player_collision)
                        .before(check_player_food_collision),
                    check_player_food_collision,
                    check_player_collision,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn create_head_mesh(size: f32) -> Mesh {
    // Two rectangles on top of each other to create a rectangle with an outline.
    let s = size / 2.0 + 2.5; // outer size
    let t = size / 2.0 - 2.5; // inner size
    let o = LinearRgba::from(css::WHITE).to_f32_array(); // outer color
    let i = LinearRgba::from(css::GREEN).to_f32_array(); // inner color
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-s, -s, 0.0],
            [s, -s, 0.0],
            [s, s, 0.0],
            [-s, s, 0.0],
            [-t, -t, 1.0],
            [t, -t, 1.0],
            [t, t, 1.0],
            [-t, t, 1.0],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![
            o, o, o, o, //
            i, i, i, i, //
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        0, 1, 3, //
        1, 2, 3, //
        4, 5, 7, //
        5, 6, 7, //
    ]))
}

fn init(
    config: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(InputQueue::default());
    commands.insert_resource(TickTimer(Timer::from_seconds(0.2, TimerMode::Repeating)));
    commands.insert_resource(Tail::default());

    let assets = PlayerAssets {
        head_mesh: meshes.add(create_head_mesh(config.pixels_per_cell as f32 - 3.0)),
        head_material: materials.add(ColorMaterial::default()),
        tail_mesh: meshes.add(Rectangle::new(
            config.pixels_per_cell as f32 - 3.0,
            config.pixels_per_cell as f32 - 3.0,
        )),
        tail_material: materials.add(Color::from(css::LIMEGREEN)),
    };
    commands.insert_resource(assets.clone());
    commands.spawn(PlayerBundle::new(&assets));
}

/// Marker to identify the player entity, the head of the snake.
#[derive(Component)]
struct Player;

/// Marker to identify the segments of the snake other than its head.
#[derive(Component)]
struct TailSegment;

/// If non-zero, grow the snake by this many segments.
#[derive(Component)]
struct SegmentsToGrow(u32);

/// A list of all snake segments excluding its head, oldest segment first.
#[derive(Default, Resource)]
struct Tail {
    segments: VecDeque<Entity>,
}

#[derive(Resource)]
struct TickTimer(Timer);

#[derive(Resource, Clone)]
struct PlayerAssets {
    head_mesh: Handle<Mesh>,
    head_material: Handle<ColorMaterial>,
    tail_mesh: Handle<Mesh>,
    tail_material: Handle<ColorMaterial>,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    pos: Position,
    z_layer: ZLayer,
    velocity: Velocity,
    segments_to_grow: SegmentsToGrow,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
}

impl PlayerBundle {
    fn new(assets: &PlayerAssets) -> PlayerBundle {
        PlayerBundle {
            player: Player,
            pos: Position { x: 5, y: 5 },
            z_layer: ZLayer { z: 10 },
            velocity: Velocity { dir: Dir::Right },
            segments_to_grow: SegmentsToGrow(3),
            mesh: Mesh2d(assets.head_mesh.clone()),
            material: MeshMaterial2d(assets.head_material.clone()),
        }
    }
}

fn spawn_segment(pos: Position, tail: &mut Tail, commands: &mut Commands, assets: &PlayerAssets) {
    let segment_id = commands
        .spawn((
            Mesh2d(assets.tail_mesh.clone()),
            MeshMaterial2d(assets.tail_material.clone()),
            TailSegment,
            pos,
            ZLayer { z: 8 },
        ))
        .id();
    tail.segments.push_back(segment_id);
}

#[derive(Default, Resource)]
struct InputQueue {
    queue: VecDeque<Dir>,
}

impl InputQueue {
    fn pop_last_input(&mut self) -> Option<Dir> {
        self.queue.pop_front()
    }
    fn insert_input(&mut self, dir: Dir) {
        self.queue.push_back(dir);
    }
}

fn read_player_input(keys: Res<ButtonInput<KeyCode>>, mut input_queue: ResMut<InputQueue>) {
    if keys.just_pressed(KeyCode::ArrowRight) {
        input_queue.insert_input(Dir::Right);
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        input_queue.insert_input(Dir::Left);
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        input_queue.insert_input(Dir::Up);
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        input_queue.insert_input(Dir::Down);
    }
}

fn apply_player_input(
    time: Res<Time>,
    mut input_queue: ResMut<InputQueue>,
    mut timer: ResMut<TickTimer>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let mut velocity = query.single_mut().unwrap();
    loop {
        let input = match input_queue.pop_last_input() {
            None => return,
            Some(d) => d,
        };
        if input != velocity.dir && input != velocity.dir.opposite() {
            velocity.dir = input;
            return;
        }
    }
}

fn move_player(
    timer: ResMut<TickTimer>,
    mut query: Query<(&mut Position, &Velocity, &mut SegmentsToGrow), With<Player>>,
    mut tail: ResMut<Tail>,
    mut commands: Commands,
    assets: Res<PlayerAssets>,
) {
    if !timer.0.just_finished() {
        return;
    }
    let (mut pos, velocity, mut to_grow) = query.single_mut().unwrap();
    if velocity.dir != Dir::None {
        spawn_segment(pos.clone(), &mut tail, &mut commands, &assets);
        if to_grow.0 == 0 {
            commands
                .entity(tail.segments.pop_front().unwrap())
                .despawn();
        } else {
            to_grow.0 -= 1;
        }
    }
    pos.apply_offset(&velocity.dir);
}

fn check_player_food_collision(
    mut player: Query<(&Position, &mut SegmentsToGrow), With<Player>>,
    food_query: Query<(Entity, &Position), With<crate::food::Food>>,
    mut commands: Commands,
    mut score: MessageWriter<crate::score::ScoreUpdate>,
) {
    let (player_pos, mut to_grow) = player.single_mut().unwrap();
    for (food, food_pos) in food_query.iter() {
        if player_pos == food_pos {
            to_grow.0 += 2;
            commands.entity(food).despawn();
            score.write(crate::score::ScoreUpdate::AteFood);
        }
    }
}

fn check_player_collision(
    mut next_state: ResMut<NextState<GameState>>,
    config: Res<Config>,
    player: Query<&Position, With<Player>>,
    segment_query: Query<&Position, With<TailSegment>>,
) {
    let pos = player.single().unwrap();
    if pos.x < 0
        || pos.x >= config.grid_size_x
        || pos.y < 0
        || pos.y >= config.grid_size_y
        || segment_query.iter().any(|p| p == pos)
    {
        next_state.set(GameState::GameOver);
    }
}
