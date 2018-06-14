use components::{Mobile, Player, Position};
use resources::{Direction, InputEvents};
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};
use tcod::input;

pub struct HandleInput;

impl<'a> System<'a> for HandleInput {
    type SystemData = (
        Write<'a, InputEvents>,
        WriteStorage<'a, Mobile>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut events, mut mobile, player): Self::SystemData) {
        let event = &mut events.0.pop();
        //println!("hmmmmm>>{:?}", event);
        println!(">>>");
        for (_pla, mob) in (&player, &mut mobile).join() {
            println!("blah");
            println!("meh {:?}", event);
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
                _ => mob.direction = Direction::Still,
            }
        }
    }
}

pub struct Motion;

impl<'a> System<'a> for Motion {
    type SystemData = (WriteStorage<'a, Mobile>, WriteStorage<'a, Position>);

    fn run(&mut self, (mut mobile, mut position): Self::SystemData) {
<<<<<<< HEAD
        println!("<<<");
        for (mob, pos) in (&mut mobile, &mut position).join() {
            match mob.direction {
                Direction::N => pos.x += 1,
                Direction::S => pos.x -= 1,
                Direction::W => pos.y += 1,
                Direction::E => pos.y -= 1,
                _ => {}
            }
            mob.direction = Direction::Still;
            println!("Yowza>>{}:{}", pos.x, pos.y);
        }
    }
=======
        for (mob, pos) in (&mut mobile, &mut position).join() {
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            match mob.direction{
                Direction::N => x += 1,
                Direction::E => y += 1,
                Direction::S => x -= 1,
                Direction::W => y -= 1,
                _ => {}
            }
            pos.x += x;
            pos.y += y;
            mob.direction = Direction::Still;
        }
    }
}

pub struct Render;

impl<'a> System<'a> for Render{
    type SystemData = (Write<'a, Console>);
>>>>>>> d6e600fefaf6782f18bba2fead73044ecafbceff
}
