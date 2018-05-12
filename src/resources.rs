use tcod::input::Event;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const LIMIT_FPS: i32 = 30;

pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    Still,
}

pub struct Input {
    pub direction: Direction,
    pub action: bool,
    pub exit: bool,
}

impl Input {
    pub fn neutral() -> Input {
        Input {
            direction: Direction::Still,
            action: false,
            exit: false,
        }
    }

    pub fn exit() -> Input {
        Input {
            direction: Direction::Still,
            action: false,
            exit: true,
        }
    }
}
