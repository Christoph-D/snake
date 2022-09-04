use crate::config::Dir;
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Default)]
pub struct InputQueue {
    queue: VecDeque<Dir>,
}

impl InputQueue {
    pub fn pop_last_input(&mut self) -> Option<Dir> {
        self.queue.pop_front()
    }
    pub fn insert_input(&mut self, dir: Dir) {
        self.queue.push_back(dir);
    }
}

pub fn read_player_input(keys: Res<Input<KeyCode>>, mut input_queue: ResMut<InputQueue>) {
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
