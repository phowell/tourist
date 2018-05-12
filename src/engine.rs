use specs::{Dispatcher, DispatcherBuilder, World};
use tcod::{colors, system, console::{blit, FontLayout, FontType, Offscreen, Root},
           input::{Event, KeyPressFlags}};

use components::{Mobile, Player, Position, Renderable};
use resources::{Direction, Input, LIMIT_FPS, SCREEN_HEIGHT, SCREEN_WIDTH};

struct InputEvents(Vec<Event>);

pub struct Engine<'a> {
    world: World,
    dispatcher: Dispatcher<'a, 'a>,
    pub root: Root,
    pub console: Offscreen,
    pub running: bool,
}

impl<'a> Engine<'a> {
    pub fn new() -> Engine<'a> {
        let mut root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust/libtcod tutorial")
            .init();
        let mut console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        let mut world = World::new();

        //components
        world.register::<Position>();
        world.register::<Renderable>();
        world.register::<Mobile>();
        world.register::<Player>();

        //resources

        world.add_resource(InputEvents(Vec::new()));

        let dispatcher = DispatcherBuilder::new().build();
        //todo: add .add(system_name) as needed between new() and build();

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
            self.render();
            self.root.flush();
            self.update();
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

    fn handle_keys(root: &mut Root) -> Option<Input> {
        use tcod::input;
        use tcod::input::{Key, KeyCode::*, KeyPressFlags};

        let key = root.check_for_keypress(KeyPressFlags::empty());
        match key {
            Some(Key {
                code: Enter,
                alt: true,
                ..
            }) => {
                // Alt+Enter: toggle fullscreen
                let fullscreen = root.is_fullscreen();
                root.set_fullscreen(!fullscreen);
                None
            }
            Some(Key { code: Escape, .. }) => Some(Input::exit()), // exit game

            // movement keys
            Some(Key { code: Up, .. }) => Some(Input {
                direction: Direction::N,
                action: false,
                exit: false,
            }),
            Some(Key { code: Down, .. }) => Some(Input {
                direction: Direction::S,
                action: false,
                exit: false,
            }),

            None => None,

            _ => None,
        }
    }
}
