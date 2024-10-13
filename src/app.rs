use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    DefaultTerminal,
};
use std::{
    fmt,
    ops::{
        Add,
        ControlFlow::{self, Break, Continue},
        Index, IndexMut, Mul, Neg, Sub,
    },
};

use crate::enums::*;

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl MenuItem {
    fn next(&self) -> Self {
        match self {
            Self::Play => Self::Options,
            Self::Options => Self::Quit,
            Self::Quit => Self::Play,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Self::Play => Self::Quit,
            Self::Quit => Self::Options,
            Self::Options => Self::Play,
        }
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub next_level: usize,
}

pub struct Level {
    pub player_location: Coord,
    pub level_state: Grid<Cell>,
    pub move_counter: u16,
}

#[derive(Clone, Copy)]
pub struct Coord(u16, u16);

#[derive(Clone, Copy)]
pub struct Offset(i16, i16);

pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.0 as usize][index.1 as usize]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.0[index.0 as usize][index.1 as usize]
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            let Ok(Event::Key(key)) = event::read() else {
                continue;
            };
            match self.process_input(key) {
                Continue(()) => {}
                Break(b) => return Ok(()),
            };
        }
        Ok(())
    }

    fn process_input(&mut self, key: KeyEvent) -> ControlFlow<bool> {
        use CurrentScreen::*;
        use KeyBind::*;
        match &self.current_screen {
            Menu(menu_item) => {
                match key.into() {
                    Quit => return Break(false),
                    Up => {
                        self.current_screen = Menu(menu_item.prev());
                    }
                    Down => {
                        self.current_screen = Menu(menu_item.next());
                    }
                    Select => match menu_item {
                        MenuItem::Quit => return Break(false),
                        MenuItem::Options => {}
                        MenuItem::Play => {
                            self.current_screen = Game(self.select_level(self.next_level));
                            self.next_level += 1;
                        }
                    },
                    _ => {}
                }
                Continue(())
            }
            Game(_) => {
                match key.into() {
                    Up | Down | Left | Right => self.move_player(key.into()),
                    Quit => return Break(false),
                    _ => {}
                }
                Continue(())
            }
        }
    }

    fn move_player(&mut self, direction: KeyBind) {
        let CurrentScreen::Game(level) = &mut self.current_screen else {
            return;
        };

        use KeyBind::*;
        let dir = Offset(
            match direction {
                Up => -1,
                Down => 1,
                _ => 0,
            },
            match direction {
                Left => -1,
                Right => 1,
                _ => 0,
            },
        );

        let next_pos = level.player_location + dir;
        let next_next_pos = level.player_location + dir * 2;
        use Cell::*;
        match (
            &level.level_state[next_pos],
            &level.level_state[next_next_pos],
        ) {
            (Empty, _) => {
                level.level_state[level.player_location] = Empty;
                level.level_state[next_pos] = Player;
            }
            (Box, Empty) => {
                level.level_state[next_pos] = Empty;
                level.level_state[next_next_pos] = Box;
            }
            _ => {}
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    fn select_level(&mut self, level: usize) -> Level {
        use Cell::{Box as B, Empty as E, Player as P, Target as T, Wall as W};
        match level {
            0 => Grid(vec![
                vec![W, W, W, W, W, W, W],
                vec![W, E, E, E, E, E, W],
                vec![W, E, P, T, B, E, W],
                vec![W, E, E, E, E, E, W],
                vec![W, W, W, W, W, W, W],
            ])
            .into(),
            1 => {
                todo!()
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_screen: CurrentScreen::Menu(MenuItem::Play),
            next_level: 0,
        }
    }
}

impl From<Grid<Cell>> for Level {
    fn from(value: Grid<Cell>) -> Self {
        Level {
            player_location: value.find_player(),
            level_state: value,
            move_counter: 0,
        }
    }
}

impl Grid<Cell> {
    fn bounds(&self) -> (u16, u16) {
        (self.0.len() as u16, self.0[0].len() as u16)
    }

    fn find_player(&self) -> Coord {
        for (i, val) in self.0.iter().enumerate() {
            for (j, who) in val.iter().enumerate() {
                if let Cell::Player = who {
                    return Coord(i as u16, j as u16);
                }
            }
        }
        Coord(0, 0)
    }
}

impl From<KeyEvent> for KeyBind {
    fn from(value: KeyEvent) -> Self {
        use KeyCode::*;
        use KeyEventKind::*;

        if let (KeyModifiers::CONTROL, Char('c')) = (value.modifiers, value.code) {
            return KeyBind::Quit;
        }

        if value.kind == Press {
            match value.code {
                Esc | Char('q') => KeyBind::Quit,
                Up | Char('w') => KeyBind::Up,
                Left | Char('a') => KeyBind::Left,
                Down | Char('s') => KeyBind::Down,
                Right | Char('d') => KeyBind::Right,
                Enter | Char(' ') => KeyBind::Select,
                _ => KeyBind::None,
            }
        } else {
            KeyBind::None
        }
    }
}

impl Add<Offset> for Coord {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        let x = if rhs.0 > 0 {
            self.0 + rhs.0 as u16
        } else {
            self.0 - rhs.0.abs() as u16
        };
        let y = if rhs.1 > 0 {
            self.1 + rhs.1 as u16
        } else {
            self.1 - rhs.1.abs() as u16
        };
        Self(x, y)
    }
}

impl Sub<Offset> for Coord {
    type Output = Self;

    fn sub(self, rhs: Offset) -> Self::Output {
        self.add(-rhs)
    }
}

impl Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl Mul<i16> for Offset {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self(self.0.saturating_mul(rhs), self.1.saturating_mul(rhs))
    }
}
