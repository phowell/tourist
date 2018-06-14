use components::{Mobile, Player, Position, Renderable};
use resources::{ActionQueue, Direction, InputEvents, LIMIT_FPS, SCREEN_HEIGHT, SCREEN_WIDTH};
use specs::{Dispatcher, DispatcherBuilder, Join, World};
use std::collections::VecDeque;
use systems::{HandleInput, Motion};
<<<<<<< HEAD
use tcod::{colors,
           console::{blit, FontLayout, FontType, Offscreen, Root},
           input,
           system,
           Console};
=======
>>>>>>> d6e600fefaf6782f18bba2fead73044ecafbceff

pub struct Engine<'a> {
    world: World,
    dispatcher: Dispatcher<'a, 'a>,
    pub root: Root,
    pub console: Offscreen,
    pub running: bool,
}

impl<'a> Engine<'a> {
    pub fn new() -> Engine<'a> {
        let root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust/libtcod tutorial")
            .init();
        let console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        let mut world = World::new();

        //components
        world.register::<Position>();
        world.register::<Renderable>();
        world.register::<Mobile>();
        world.register::<Player>();

        //resources
        world.add_resource(InputEvents(Vec::new()));
        world.add_resource(ActionQueue(VecDeque::new()));

        let dispatcher = DispatcherBuilder::new()
            .with(HandleInput, "HandleInput", &[])
            .with(Motion, "Motion", &["HandleInput"])
<<<<<<< HEAD
            //todo: add .with(system_name, 'systemName' &[deps]) as needed between new() and build();
=======
>>>>>>> d6e600fefaf6782f18bba2fead73044ecafbceff
            .build();

        world
            .create_entity()
            .with(Position { x: 1, y: 1 })
            .with(Mobile {
                direction: Direction::Still,
            })
            .with(Player)
            .with(Renderable {
                color: colors::WHITE,
                character: '@',
            });

        let running = true;

        Engine {
            world,
            dispatcher,
            root,
            console,
            running,
        }
    }
    pub fn run(&mut self) {
        system::set_fps(LIMIT_FPS);

        while self.running {
            self.root.flush();
            self.running = !self.root.window_closed();
            self.handle_input();
            self.update();
            self.render();
        }
    }

    fn render(&mut self) {
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();
        //println!("testing>>>>");
        for (pos, rend) in (&positions, &renderables).join() {
            self.console
                .put_char_ex(pos.x, pos.y, rend.character, rend.color, colors::WHITE);
            println!(">>Player{}:{}", pos.x, pos.y);
        }

        blit(
            &mut self.console,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );
    }
    fn update(&mut self) {
        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();

    }

    fn handle_input(&mut self) {
        let new_input = input::check_for_event(input::KEY_PRESS);
        match new_input {
            //Did you just hit Esc?
            Some((
                _,
                input::Event::Key(input::Key {
                    code: input::KeyCode::Escape,
                    ..
                }),
            )) => self.running = false,

            //Every other input
            Some(e) => {
                println!(".....{:?}", e.1);
                let x = self.world.write_resource::<InputEvents>().0.push(e.1);
                //println!("{:?}", e.1);
                x
            }

            //No input occurred
            _ => {}
        }
    }
}
