use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    DefaultTerminal,
};
use std::{
    borrow::BorrowMut,
    fmt,
    ops::{
        Add,
        ControlFlow::{self, Break, Continue},
        Index, IndexMut, Mul, Neg, RemAssign, Sub,
    },
    time::{Duration, Instant, SystemTime},
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
    pub timing_buffer: [Duration; 30],
    pub timing_index: u8,
}

pub struct Level {
    pub player_location: Coord,
    pub level_state: Grid<Cell>,
    pub move_counter: usize,
    pub remaining_boxes: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coord(u16, u16);

#[derive(Clone, Copy)]
pub struct Offset(i16, i16);

#[derive(Debug)]
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
        let target_fps = 60.0;
        let mut now = Instant::now();
        let mut delta = now.elapsed();
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            match event::poll(Duration::from_secs_f64(1f64 / target_fps).saturating_sub(delta)) {
                Ok(true) => {
                    let Ok(Event::Key(key)) = event::read() else {
                        unreachable!()
                    };
                    match self.process_input(key) {
                        Continue(()) => {}
                        Break(b) => return Ok(()),
                    };
                }
                _ => {}
            };

            delta = now.elapsed();
            now = Instant::now();

            self.timing_buffer[self.timing_index as usize] = delta;
            self.timing_index += 1;
            self.timing_index %= 30;

            self.update(delta);
        }
        Ok(())
    }

    fn update(&mut self, delta: Duration) {
        match &self.current_screen {
            CurrentScreen::Menu(_) => {}
            CurrentScreen::Game(level) => {
                if level.remaining_boxes == 0 {
                    self.next_level()
                }
            }
        }
    }

    fn next_level(&mut self) {
        self.current_screen = CurrentScreen::Game(self.select_level(self.next_level));
        self.next_level += 1;
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

        assert!(
            level.level_state[level.player_location] == Player
                || level.level_state[level.player_location] == PlayerOnTarget
        );

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

        let grid = level.level_state.borrow_mut();

        use Cell::*;
        (grid[next_pos], grid[next_next_pos]) = match (&grid[next_pos], &grid[next_next_pos]) {
            (Empty, _) => {
                grid[level.player_location] = match grid[level.player_location] {
                    Player => Empty,
                    PlayerOnTarget => Target,
                    _ => unreachable!(),
                };
                level.player_location = next_pos;
                level.move_counter += 1;
                (Player, grid[next_next_pos])
            }
            (Target, _) => {
                grid[level.player_location] = Empty;
                level.player_location = next_pos;
                level.move_counter += 1;
                (PlayerOnTarget, grid[next_next_pos])
            }
            (Box, Empty) => (Empty, Box),
            (Box, Target) => {
                level.remaining_boxes -= 1;
                (Empty, LockedBox)
            }
            (LockedBox, Empty) => {
                level.remaining_boxes += 1;
                (Target, Box)
            }
            (other, thing) => (*other, *thing),
        }
    }

    fn select_level(&mut self, level: usize) -> Level {
        use Cell::{Box as B, Empty as E, LockedBox as L, Player as P, Target as T, Wall as W};
        match level {
            0 => {
                let mut grid = Grid(vec![vec![P, T, B]]);
                grid.wrap(E);
                grid.wrap(W);
                grid.into()
            }
            1 => {
                let mut grid = Grid(vec![
                    vec![E, E, W, W, W, W, W, E],
                    vec![W, W, W, E, E, E, W, E],
                    vec![W, T, P, B, E, E, W, E],
                    vec![W, W, W, E, B, T, W, E],
                    vec![W, T, W, W, B, E, W, E],
                    vec![W, E, W, E, T, E, W, W],
                    vec![W, B, E, L, B, B, T, W],
                    vec![W, E, E, E, T, E, E, W],
                    vec![W, W, W, W, W, W, W, W],
                ]);
                grid.into()
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
            timing_buffer: [Duration::ZERO; 30],
            timing_index: 0,
        }
    }
}

impl From<Grid<Cell>> for Level {
    fn from(mut value: Grid<Cell>) -> Self {
        value.wrap(Cell::Empty);
        let Plural::One(player_location) = value.get(Cell::Player) else {
            panic!()
        };
        Level {
            player_location,
            remaining_boxes: value.count(Cell::Box),
            level_state: value,
            move_counter: 0,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    fn bounds(&self) -> (u16, u16) {
        (self.0.len() as u16, self.0[0].len() as u16)
    }

    fn wrap(&mut self, with: T) {
        let b = self.bounds();
        let bar = vec![with; b.1 as usize + 2];
        for mut i in &mut self.0 {
            i.insert(0, with);
            while i.len() < b.1 as usize + 2 {
                i.push(with);
            }
        }
        self.0.insert(0, bar.clone());
        self.0.push(bar);
    }

    fn get(&self, getting: T) -> Plural<Coord> {
        let mut out = vec![];
        for (i, val) in self.0.iter().enumerate() {
            for (j, who) in val.iter().enumerate() {
                if getting == *who {
                    out.push(Coord(i as u16, j as u16));
                }
            }
        }
        if out.is_empty() {
            Plural::None
        } else if out.len() == 1 {
            Plural::One(out[0])
        } else {
            Plural::Many(out)
        }
    }

    fn count(&self, getting: T) -> usize {
        match self.get(getting) {
            Plural::None => 0,
            Plural::One(_) => 1,
            Plural::Many(v) => v.len(),
        }
    }
}

enum Plural<T> {
    None,
    One(T),
    Many(Vec<T>),
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
