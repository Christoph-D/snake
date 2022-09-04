use crate::config::*;
use crate::input::InputQueue;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod camera;
mod config;
mod game_over;
mod grid;
mod input;

struct TickTimer(Timer);

#[derive(Component)]
struct Player;

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
    commands.spawn_bundle(grid::GridBundle::from_config(&config));
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

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins_with(DefaultPlugins, |plugins| {
            plugins.disable::<bevy::audio::AudioPlugin>()
        })
        .add_plugin(ShapePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(game_over::GameOverScreenPlugin)
        .add_state(GameState::InGame)
        .insert_resource(Config {
            grid_size_x: 25,
            grid_size_y: 25,
            pixels_per_cell: 30,
        })
        .insert_resource(InputQueue::default())
        .insert_resource(TickTimer(Timer::from_seconds(0.2, true)))
        .add_system(bevy::window::close_on_esc)
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset_game))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(input::read_player_input.before(apply_player_input))
                .with_system(apply_player_input.before(move_all))
                .with_system(move_all.before(check_player_collision))
                .with_system(check_player_collision.before(update_transformations))
                .with_system(update_transformations),
        )
        .run();
}
