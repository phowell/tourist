use specs::Entity;
use std::collections::VecDeque;
use tcod::input::Event;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const LIMIT_FPS: i32 = 30;

#[derive(Default)]
pub struct InputEvents(pub Vec<Event>);

#[derive(Default)]
pub struct ActionQueue(pub VecDeque<(Entity, Direction)>);

#[derive(Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
    Still,
}
