use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

struct TickTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct PrimaryColor(Color);

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Velocity {
    x: i32,
    y: i32,
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
    commands
        .spawn()
        .insert(Name("player".to_owned()))
        .insert(Player)
        .insert(Position { x: 5, y: 5 })
        .insert(Velocity { x: 1, y: 0 });
}

fn move_all(
    time: Res<Time>,
    mut timer: ResMut<TickTimer>,
    mut query: Query<(&mut Position, &Velocity)>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for (mut pos, velocity) in query.iter_mut() {
        pos.x += velocity.x;
        pos.y += velocity.y;
    }
}

fn print(query: Query<(&Name, &Position), Changed<Position>>) {
    for (name, pos) in query.iter() {
        println!("{}: {}:{}", name.0, pos.x, pos.y)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TickTimer(Timer::from_seconds(1.0, true)))
        .add_startup_system(init)
        .add_system(move_all)
        .add_system(print)
        .run();
}
