use strum::EnumIter;

use crate::app::Level;

pub enum KeyBind {
    Quit,
    Up,
    Down,
    Left,
    Right,
    Select,
    None,
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

#[derive(PartialEq, Eq)]
pub enum Cell {
    Empty,
    Player,
    Box,
    Wall,
    Target,
}
