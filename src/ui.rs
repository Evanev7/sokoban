use std::fmt;
use std::time::Duration;

use crate::app::*;
use crate::enums::*;
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{block::Title, Block, BorderType, Padding, Paragraph, Wrap},
    Frame,
};
use strum::IntoEnumIterator;

impl App {
    pub fn draw(&self, frame: &mut Frame) {
        use CurrentScreen::*;
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(Title::from(Span::styled(
                match &self.current_screen {
                    Menu(_) => "Sokoban!".to_owned(),
                    Game(level) => format!(
                        "Level{}: {} Moves: {} Boxes Remaining ",
                        " 1",
                        level.move_counter.to_string(),
                        level.remaining_boxes
                    ),
                },
                Style::default().fg(Color::Green),
            )))
            .padding(Padding::uniform(1))
            .title_alignment(Alignment::Left)
            .title_bottom(format!(
                "FPS:{:.0}",
                1.0 / self
                    .timing_buffer
                    .into_iter()
                    .reduce(std::ops::Add::add)
                    .unwrap()
                    .div_f64(30.0)
                    .as_secs_f64()
            ))
            .title_bottom("do some keybinds");

        let focused_style = Style::default().add_modifier(Modifier::BOLD);
        let unfocused_style = Style::default();

        match &self.current_screen {
            Menu(focused_item) => {
                let lines: Vec<_> = MenuItem::iter()
                    .map(|item| {
                        Line::styled(
                            item.to_string(),
                            if focused_item == &item {
                                focused_style
                            } else {
                                unfocused_style
                            },
                        )
                    })
                    .collect();

                let menu_block = Paragraph::new(lines)
                    .block(block)
                    .centered()
                    .wrap(Wrap { trim: false });

                frame.render_widget(menu_block, frame.area());
            }
            Game(level) => {
                let lines: Vec<_> = level
                    .level_state
                    .0
                    .iter()
                    .map(|row| {
                        Line::from(row.iter().map(|cell| cell.to_span()).collect::<Vec<_>>())
                    })
                    .collect();

                let game_block = Paragraph::new(lines).block(block).centered();

                frame.render_widget(game_block, frame.area());
            }
        }
    }
}

impl Cell {
    fn to_string(&self) -> &str {
        use Cell::*;
        match self {
            Player {
                on_target: false,
                hp,
            } => "@@",
            Player {
                on_target: true,
                hp,
            } => "@<",
            Box { locked: false } => "[]",
            Empty => "  ",
            Wall => "██",
            Target => "><",
            Box { locked: true } => "░░",
            Turret {
                direction,
                cooldown: _,
            } => match direction {
                Direction::Up => "▟▙",
                Direction::Left => "┫█",
                Direction::Right => "█┣",
                Direction::Down => "▜▛",
            },
            Bullet {
                direction,
                on_target,
            } => "🞀🞂",
        }
    }

    fn to_span(&self) -> Span {
        self.to_string().into()
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
