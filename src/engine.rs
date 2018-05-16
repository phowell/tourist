use specs::{Dispatcher, DispatcherBuilder, World};
use tcod::{colors, input, system, console::{blit, FontLayout, FontType, Offscreen, Root},
           input::{Event, EventFlags}};

use components::{Mobile, Player, Position, Renderable};
use resources::{LIMIT_FPS, SCREEN_HEIGHT, SCREEN_WIDTH};
use systems::HandleInput;

pub struct InputEvents(Vec<Event>);

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

        let dispatcher = DispatcherBuilder::new()
            .with(HandleInput, "HandleInput", &[])
            .build();
        //todo: add .with(system_name) as needed between new() and build();

        world
            .create_entity()
            .with(Position { x: 1, y: 1 })
            .with(Mobile)
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
            self.running = !self.root.window_closed();
            self.handle_input();
            self.update();
            self.render();
            self.root.flush();
        }
    }

    fn render(&mut self) {
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
    fn update(&mut self) {}

    fn handle_input(&mut self) {
        let new_input = input::check_for_event(EventFlags::all());
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
            Some(e) => self.world.write_resource::<InputEvents>().0.push(e.1),

            //No input occurred
            _ => {}
        }
    }
}
