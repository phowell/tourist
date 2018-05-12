use specs::{Component, NullStorage, VecStorage};
use tcod::colors::Color;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Renderable {
    pub character: char,
    pub color: Color,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Mobile;

impl Component for Mobile {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}
