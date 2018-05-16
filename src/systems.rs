use engine::InputEvents;
use resources::{Direction, Input};
use specs::Read;
use specs::System;
use tcod::input;

pub struct HandleInput;

impl<'a, InputEvents> System<'a> for HandleInput {
    type SystemData = (Read<'a, InputEvents>);

    fn run(&mut self, data: Self::SystemData) {
        match data.0.pop().1 {
            Some(input::Event::Key(input::Key {
                code: input::KeyCode::Up,
                ..
            })) => SEND_THIS_TO_PLAYER_AVATAR(Input {
                direction: Direction::N,
                action: false,
            }),
            _ => {}
        }
    }
}
