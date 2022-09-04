use crate::config::*;
use crate::input::InputQueue;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::collections::VecDeque;

mod camera;
mod config;
mod food;
mod game_over;
mod grid;
mod input;

struct TickTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct TailSegment;

/// If non-zero, grow the snake by this many segments.
#[derive(Component)]
struct SegmentsToGrow(u32);

#[derive(Default)]
struct Tail {
    segments: VecDeque<Entity>,
}

fn reset_game(
    config: Res<Config>,
    all_except_camera: Query<Entity, Without<Camera2d>>,
    mut commands: Commands,
) {
    for entity in all_except_camera.iter() {
        commands.entity(entity).despawn();
    }

    commands.insert_resource(InputQueue::default());
    commands.insert_resource(TickTimer(Timer::from_seconds(0.2, true)));
    commands.insert_resource(Tail::default());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::RegularPolygon {
                sides: 4,
                feature: shapes::RegularPolygonFeature::SideLength(
                    config.pixels_per_cell as f32 - 3.0,
                ),
                ..shapes::RegularPolygon::default()
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GREEN),
                outline_mode: StrokeMode::new(Color::WHITE, 5.0),
            },
            Transform::default(),
        ))
        .insert(Name("player".to_owned()))
        .insert(Player)
        .insert(Position { x: 5, y: 5 })
        .insert(ZLayer { z: 10 })
        .insert(Velocity { dir: Dir::Right })
        .insert(SegmentsToGrow(3));
    commands.spawn_bundle(grid::GridBundle::from_config(&config));
}

fn spawn_segment(config: &Config, pos: Position, tail: &mut Tail, commands: &mut Commands) {
    let z_layer = ZLayer { z: 8 };
    let segment_id = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::RegularPolygon {
                sides: 4,
                feature: shapes::RegularPolygonFeature::SideLength(
                    config.pixels_per_cell as f32 - 3.0,
                ),
                ..shapes::RegularPolygon::default()
            },
            DrawMode::Fill(FillMode::color(Color::LIME_GREEN)),
            Transform::default(),
        ))
        .insert(TailSegment)
        .insert(pos)
        .insert(z_layer)
        .id();
    tail.segments.push_back(segment_id);
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
    let mut velocity = query.single_mut();
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
    config: Res<Config>,
    mut tail: ResMut<Tail>,
    mut commands: Commands,
) {
    if !timer.0.just_finished() {
        return;
    }
    let (mut pos, velocity, mut to_grow) = query.single_mut();
    if velocity.dir != Dir::None {
        spawn_segment(&config, pos.clone(), &mut tail, &mut commands);
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
    food_query: Query<(Entity, &Position), (Without<Player>, With<food::Food>)>,
    mut commands: Commands,
) {
    let (player_pos, mut to_grow) = player.single_mut();
    for (food, food_pos) in food_query.iter() {
        if player_pos == food_pos {
            to_grow.0 += 2;
            commands.entity(food).despawn();
        }
    }
}

fn check_player_collision(
    mut game_state: ResMut<State<GameState>>,
    config: Res<Config>,
    mut player: Query<&Position, With<Player>>,
    segment_query: Query<&Position, (Without<Player>, With<TailSegment>)>,
) {
    let pos = player.single_mut();
    if pos.x < 0
        || pos.x >= config.grid_size_x
        || pos.y < 0
        || pos.y >= config.grid_size_y
        || segment_query.iter().any(|p| p == &*pos)
    {
        game_state.set(GameState::GameOver).unwrap();
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

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake".to_owned(),
            canvas: Some("#game".to_owned()),
            fit_canvas_to_parent: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(game_over::GameOverScreenPlugin)
        .add_plugin(food::FoodPlugin)
        .add_state(GameState::InGame)
        .insert_resource(Config {
            grid_size_x: 20,
            grid_size_y: 20,
            pixels_per_cell: 30,
        })
        .add_system(bevy::window::close_on_esc)
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset_game))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(input::read_player_input.before(apply_player_input))
                .with_system(apply_player_input.before(move_player))
                .with_system(
                    move_player
                        .before(check_player_collision)
                        .before(check_player_food_collision),
                )
                .with_system(check_player_food_collision)
                .with_system(check_player_collision)
        )
        .add_stage_after(CoreStage::Update, "update_transformations", SystemStage::parallel())
        .add_system_to_stage("update_transformations", update_transformations)
        .run();
}
