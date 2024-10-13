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
                    Menu(_) => " Sokoban! ".to_owned(),
                    Game(level) => format!(" Level{}:{} ", " 1 ", level.move_counter.to_string()),
                },
                Style::default().fg(Color::Green),
            )))
            .padding(Padding::uniform(1))
            .title_alignment(Alignment::Left)
            .title_bottom("do some keybinds bozo");

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
    fn to_span(&self) -> Span {
        use Cell::*;
        match self {
            Player => "@@".into(),
            Box => "[]".into(),
            Empty => "  ".into(),
            Wall => "██".into(),
            Target => "┥┝".into(),
        }
    }
}
