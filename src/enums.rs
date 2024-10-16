use strum::EnumIter;

use crate::app::{Coord, Level, Offset};

pub enum KeyBind {
    Quit,
    Up,
    Down,
    Left,
    Right,
    Select,
    None,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<KeyBind> for Direction {
    fn from(value: KeyBind) -> Self {
        use Direction as D;
        use KeyBind::*;
        match value {
            Up => D::Up,
            Down => D::Down,
            Left => D::Left,
            Right => D::Right,
            _ => panic!("Shouldn't convert other keybind to direction."),
        }
    }
}

pub enum CurrentScreen {
    Menu(MenuItem),
    Game(Level),
}

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum MenuItem {
    Play,
    Options,
    Quit,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Empty,
    Player {
        on_target: bool,
        hp: u8,
    },
    Turret {
        direction: Direction,
        cooldown: u8,
    },
    Bullet {
        direction: Direction,
        on_target: bool,
    },
    Box {
        locked: bool,
    },
    Wall,
    Target,
}
