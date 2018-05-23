#![warn(clippy)]

extern crate specs;
extern crate tcod;

mod components;
mod engine;
mod entities;
mod resources;
mod systems;

use engine::Engine;

fn main() {
    let mut eng: Engine = Engine::new();
    eng.run();
}
