use resources::{InputEvents, Direction, Input};
use specs::Read;
use specs::System;
use tcod::input;

pub struct HandleInput;

impl<'a> System<'a> for HandleInput {
    type SystemData = (Read<'a, InputEvents>);

    fn run(&mut self, data: Self::SystemData) {
        let mut d = data.0.to_owned(); 
        println!("this should repeat");
        match d.pop() { 
            Some(input::Event::Key(input::Key {
                code: input::KeyCode::Char,
                ..
            })) => println!("test test test"), 
            _ => println!("default case"),
        }
    }
}
