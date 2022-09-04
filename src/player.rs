use crate::config::*;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use std::collections::VecDeque;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(init))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(read_player_input.before(apply_player_input))
                    .with_system(apply_player_input.before(move_player))
                    .with_system(
                        move_player
                            .before(check_player_collision)
                            .before(check_player_food_collision),
                    )
                    .with_system(check_player_food_collision)
                    .with_system(check_player_collision),
            );
    }
}

fn init(config: Res<Config>, mut commands: Commands) {
    commands.insert_resource(InputQueue::default());
    commands.insert_resource(TickTimer(Timer::from_seconds(0.2, true)));
    commands.insert_resource(Tail::default());
    commands.spawn_bundle(PlayerBundle::from_config(&config));
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
#[derive(Default)]
struct Tail {
    segments: VecDeque<Entity>,
}

struct TickTimer(Timer);

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    pos: Position,
    z_layer: ZLayer,
    velocity: Velocity,
    segments_to_grow: SegmentsToGrow,
    #[bundle]
    shape_bundle: ShapeBundle,
}

impl PlayerBundle {
    fn from_config(config: &Config) -> PlayerBundle {
        PlayerBundle {
            player: Player,
            pos: Position { x: 5, y: 5 },
            z_layer: ZLayer { z: 10 },
            velocity: Velocity { dir: Dir::Right },
            segments_to_grow: SegmentsToGrow(3),
            shape_bundle: GeometryBuilder::build_as(
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
            ),
        }
    }
}

fn spawn_segment(config: &Config, pos: Position, tail: &mut Tail, commands: &mut Commands) {
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
        .insert(ZLayer { z: 8 })
        .id();
    tail.segments.push_back(segment_id);
}

#[derive(Default)]
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

fn read_player_input(keys: Res<Input<KeyCode>>, mut input_queue: ResMut<InputQueue>) {
    if keys.just_pressed(KeyCode::Right) {
        input_queue.insert_input(Dir::Right);
    }
    if keys.just_pressed(KeyCode::Left) {
        input_queue.insert_input(Dir::Left);
    }
    if keys.just_pressed(KeyCode::Up) {
        input_queue.insert_input(Dir::Up);
    }
    if keys.just_pressed(KeyCode::Down) {
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
    food_query: Query<(Entity, &Position), (Without<Player>, With<crate::food::Food>)>,
    mut commands: Commands,
    mut score: EventWriter<crate::score::ScoreUpdate>,
) {
    let (player_pos, mut to_grow) = player.single_mut();
    for (food, food_pos) in food_query.iter() {
        if player_pos == food_pos {
            to_grow.0 += 2;
            commands.entity(food).despawn();
            score.send(crate::score::ScoreUpdate::AteFood);
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
        || segment_query.iter().any(|p| p == pos)
    {
        game_state.set(GameState::GameOver).unwrap();
    }
}
