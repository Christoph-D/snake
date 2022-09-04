use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::collections::VecDeque;

const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GameState {
    InGame,
    GameOver,
}

struct Config {
    grid_size_x: i32,
    grid_size_y: i32,
    pixels_per_cell: i32,
}

#[derive(Default)]
struct InputQueue {
    queue: VecDeque<Dir>,
}

impl InputQueue {
    fn pop_last_input(&mut self) -> Option<Dir> {
        return self.queue.pop_front();
    }
    fn insert_input(&mut self, dir: Dir) {
        self.queue.push_back(dir);
    }
}

struct TickTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Grid;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn apply_offset(&mut self, dir: &Dir) {
        let (dx, dy) = dir.to_x_y();
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Clone, PartialEq)]
enum Dir {
    None,
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn to_x_y(&self) -> (i32, i32) {
        match self {
            Dir::None => (0, 0),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
        }
    }
    fn opposite(&self) -> Dir {
        match self {
            Dir::None => Dir::None,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
        }
    }
}

#[derive(Component)]
struct Velocity {
    dir: Dir,
}

fn init(config: Res<Config>, mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.x =
        ((config.grid_size_x - 1) * config.pixels_per_cell) as f32 / 2.0;
    camera.transform.translation.y =
        ((config.grid_size_y - 1) * config.pixels_per_cell) as f32 / 2.0;
    commands.spawn_bundle(camera);
}

fn reset_game(
    config: Res<Config>,
    all_except_camera: Query<Entity, Without<Camera2d>>,
    mut commands: Commands,
) {
    for entity in all_except_camera.iter() {
        commands.entity(entity).despawn();
    }

    commands
        .spawn()
        .insert(Name("player".to_owned()))
        .insert(Player)
        .insert(Position { x: 5, y: 5 })
        .insert(Velocity { dir: Dir::None })
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::RegularPolygon {
                sides: 4,
                feature: shapes::RegularPolygonFeature::SideLength(config.pixels_per_cell as f32),
                ..shapes::RegularPolygon::default()
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GREEN),
                outline_mode: StrokeMode::new(Color::WHITE, 5.0),
            },
            Transform::default(),
        ));
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
    commands
        .spawn()
        .insert(Name("grid".to_owned()))
        .insert(Grid)
        .insert_bundle(grid_builder.build(
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 1.0)),
            Transform::default(),
        ));
}

fn show_game_over_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        align_self: AlignSelf::Center,
                        padding: UiRect::all(Val::Px(30.0)),
                        ..Default::default()
                    },
                    color: BACKGROUND_COLOR.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(
                        TextBundle::from_sections(vec![
                            TextSection {
                                value: "Game over!".to_owned(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 100.0,
                                    color: Color::RED,
                                },
                            },
                            TextSection {
                                value: "\nPress any key to restart".to_owned(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::RED,
                                },
                            },
                        ])
                        .with_text_alignment(TextAlignment::TOP_CENTER)
                        .with_style(Style {
                            align_self: AlignSelf::Center,
                            ..default()
                        }),
                    );
                });
        });
}

fn fit_to_window(
    config: Res<Config>,
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let pixels_x = (config.grid_size_x * config.pixels_per_cell) as f32 + 50.0;
    let pixels_y = (config.grid_size_y * config.pixels_per_cell) as f32 + 50.0;
    let window = windows.primary();
    let min_size = window.width().min(window.height());
    let mut transform = query.single_mut();
    transform.scale.x = pixels_x / min_size;
    transform.scale.y = pixels_y / min_size;
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

fn move_all(timer: ResMut<TickTimer>, mut query: Query<(&mut Position, &Velocity)>) {
    if !timer.0.just_finished() {
        return;
    }
    for (mut pos, velocity) in query.iter_mut() {
        pos.apply_offset(&velocity.dir);
    }
}

fn check_player_collision(
    mut game_state: ResMut<State<GameState>>,
    config: Res<Config>,
    mut query: Query<(&mut Position, &Velocity), With<Player>>,
) {
    let (mut pos, velocity) = query.single_mut();
    if pos.x < 0 || pos.x >= config.grid_size_x || pos.y < 0 || pos.y >= config.grid_size_y {
        pos.apply_offset(&velocity.dir.opposite());
        game_state.set(GameState::GameOver).unwrap();
    }
}

fn update_transformations(config: Res<Config>, mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, pos) in query.iter_mut() {
        *transform = Transform::from_xyz(
            (config.pixels_per_cell * pos.x) as f32,
            (config.pixels_per_cell * pos.y) as f32,
            1.0,
        );
    }
}

fn read_restart_input(keys: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keys.get_just_pressed().next().is_some() {
        game_state.set(GameState::InGame).unwrap();
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins_with(DefaultPlugins, |plugins| {
            plugins.disable::<bevy::audio::AudioPlugin>()
        })
        .add_plugin(ShapePlugin)
        .add_state(GameState::InGame)
        .insert_resource(Config {
            grid_size_x: 25,
            grid_size_y: 25,
            pixels_per_cell: 30,
        })
        .insert_resource(InputQueue::default())
        .insert_resource(TickTimer(Timer::from_seconds(0.2, true)))
        .add_startup_system(init)
        .add_system(bevy::window::close_on_esc)
        .add_system(fit_to_window)
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset_game))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(read_player_input.before(apply_player_input))
                .with_system(apply_player_input.before(move_all))
                .with_system(move_all.before(check_player_collision))
                .with_system(check_player_collision.before(update_transformations))
                .with_system(update_transformations),
        )
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(show_game_over_screen))
        .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(read_restart_input))
        .run();
}
