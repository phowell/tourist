use resources::{InputEvents, Direction, Input};
use specs::Write;
use specs::System;
use tcod::input;

pub struct HandleInput;

impl<'a> System<'a> for HandleInput {
    type SystemData = (Write<'a, InputEvents>);

    fn run(&mut self, mut events: Self::SystemData) {
        let e = &mut events.0;

        match e.pop() { 
            Some(input::Event::Key(input::Key {
                code: input::KeyCode::Up, 
                ..
            })) => println!("up"),
            Some(input::Event::Key(input::Key {
                code: input::KeyCode::Down,
                ..
            })) => println!("down"),
            Some(input::Event::Key(input::Key {
                code: input::KeyCode::Left,
                ..
            })) => println!("left"),
            Some(input::Event::Key(input::Key {
                code: input::KeyCode::Right,
                ..
            })) => println!("right"), 
            _ => {}
        }
    }
}
