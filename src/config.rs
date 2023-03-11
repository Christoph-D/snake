use bevy::prelude::*;

#[derive(Resource)]
pub struct Config {
    pub grid_size_x: i32,
    pub grid_size_y: i32,
    pub pixels_per_cell: i32,
}

pub const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(States, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameState {
    #[default]
    InGame,
    GameOver,
}

#[derive(Clone, Component, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Component, Eq, PartialEq)]
pub struct ZLayer {
    pub z: i32,
}

impl Position {
    pub fn apply_offset(&mut self, dir: &Dir) {
        let (dx, dy) = dir.to_x_y();
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Dir {
    None,
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    pub fn to_x_y(&self) -> (i32, i32) {
        match self {
            Dir::None => (0, 0),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
        }
    }
    pub fn opposite(&self) -> Dir {
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
pub struct Velocity {
    pub dir: Dir,
}
