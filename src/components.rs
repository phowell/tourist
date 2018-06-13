use resources::Direction;
use specs::{Component, VecStorage};
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

#[derive(Debug)]
pub struct Mobile {
    pub direction: Direction,
}

impl Component for Mobile {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Player;

impl Component for Player {
    type Storage = VecStorage<Self>;
}
