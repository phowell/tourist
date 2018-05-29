use components::{Mobile, Player, Position};
use resources::{Direction, InputEvents};
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};
use tcod::input;

pub struct HandleInput;

impl<'a> System<'a> for HandleInput {
    type SystemData = (
        Write<'a, InputEvents>,
        WriteStorage<'a, Mobile>,
        Entities<'a>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut events, mut mobile, entities, player): Self::SystemData) {
        let event = &mut events.0.pop();
        for (_ent, _pla, mob) in (&*entities, &player, &mut mobile).join() {
            match event {
                Some(input::Event::Key(input::Key {
                    code: input::KeyCode::Up,
                    ..
                })) => mob.direction = Direction::N,

                Some(input::Event::Key(input::Key {
                    code: input::KeyCode::Down,
                    ..
                })) => mob.direction = Direction::S,
                Some(input::Event::Key(input::Key {
                    code: input::KeyCode::Left,
                    ..
                })) => mob.direction = Direction::W,
                Some(input::Event::Key(input::Key {
                    code: input::KeyCode::Right,
                    ..
                })) => mob.direction = Direction::E,
                _ => {}
            }
        }
    }
}

pub struct Motion;

impl<'a> System<'a> for Motion {
    type SystemData = (WriteStorage<'a, Mobile>, WriteStorage<'a, Position>);

    fn run(&mut self, (mut mobile, mut position): Self::SystemData) {}
}
