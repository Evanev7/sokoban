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

// use color_eyre::owo_colors::colors::Default;
// use ratatui::{
//     layout::{Constraint, Direction, Layout, Rect},
//     style::{Color, Style},
//     text::{Line, Span, Text},
//     widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
//     Frame,
// };
// use serde::de::value;

// use crate::app::{App, CurrentScreen, CurrentlyEditing};

// fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
//     let popup_layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Percentage((100 - percent_y) / 2),
//             Constraint::Percentage(percent_y),
//             Constraint::Percentage((100 - percent_y) / 2),
//         ])
//         .split(r);

//     Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Percentage((100 - percent_y) / 2),
//             Constraint::Percentage(percent_y),
//             Constraint::Percentage((100 - percent_y) / 2),
//         ])
//         .split(popup_layout[1])[1]
// }

// pub fn ui(frame: &mut Frame, app: &App) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Length(3),
//             Constraint::Min(1),
//             Constraint::Length(3),
//         ])
//         .split(frame.area());

//     let title_block = Block::default()
//         .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
//         .style(Style::default())
//         .border_type(BorderType::Rounded);

//     let title = Paragraph::new(Text::styled(
//         "Create new json",
//         Style::default().fg(Color::Green),
//     ))
//     .block(title_block);

//     frame.render_widget(title, chunks[0]);

//     let mut list_items = Vec::<ListItem>::new();

//     for (key, value) in app.pairs.iter() {
//         list_items.push(ListItem::new(Line::from(Span::styled(
//             format!("{: <25}: {}", key, value),
//             Style::default().fg(Color::Yellow),
//         ))));
//     }

//     let list = List::new(list_items);

//     frame.render_widget(list, chunks[1]);

//     let current_navigation_text = vec![
//         match app.current_screen {
//             CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
//             CurrentScreen::Editing => {
//                 Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
//             }
//             CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
//         },
//         Span::styled(" | ", Style::default().fg(Color::White)),
//         if let Some(editing) = &app.currently_editing {
//             match editing {
//                 CurrentlyEditing::Key => {
//                     Span::styled("Editing Json Key", Style::default().fg(Color::Green))
//                 }
//                 CurrentlyEditing::Value => {
//                     Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
//                 }
//             }
//         } else {
//             Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
//         },
//     ];

//     let mode_footer = Paragraph::new(Line::from(current_navigation_text)).block(
//         Block::default()
//             .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
//             .border_type(BorderType::Rounded),
//     );

//     let current_keys_hint = {
//         Span::styled(
//             match app.current_screen {
//                 CurrentScreen::Main => "(q) to quit / (e) to make new pair",
//                 CurrentScreen::Editing => "(ESC) to cancel / (Tab) to switch / enter to complete",
//                 CurrentScreen::Exiting => "(q) to quit / (e) to make new pair",
//             },
//             Style::default().fg(Color::Red),
//         )
//     };

//     let key_notes_footer = Paragraph::new(Line::from(current_keys_hint)).block(
//         Block::default()
//             .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
//             .border_type(BorderType::Rounded),
//     );

//     let footer_chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
//         .split(chunks[2]);

//     frame.render_widget(mode_footer, footer_chunks[0]);
//     frame.render_widget(key_notes_footer, footer_chunks[1]);

//     if let Some(editing) = &app.currently_editing {
//         let popup_block = Block::default()
//             .title("Etner a new key-value pair")
//             .borders(Borders::NONE)
//             .style(Style::default().bg(Color::DarkGray));

//         let area = centered_rect(60, 25, frame.area());
//         frame.render_widget(popup_block, area);

//         let popup_chunks = Layout::default()
//             .direction(Direction::Horizontal)
//             .margin(1)
//             .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
//             .split(area);

//         let mut key_block = Block::default().title("Key").borders(Borders::ALL);
//         let mut value_block = Block::default().title("Value").borders(Borders::ALL);

//         let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

//         match editing {
//             CurrentlyEditing::Key => key_block = key_block.style(active_style),
//             CurrentlyEditing::Value => value_block = value_block.style(active_style),
//         };

//         let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
//         frame.render_widget(key_text, popup_chunks[0]);

//         let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
//         frame.render_widget(value_text, popup_chunks[1]);
//     }

//     if let CurrentScreen::Exiting = app.current_screen {
//         frame.render_widget(Clear, frame.area());
//         let popup_block = Block::default()
//             .title("Y/N")
//             .borders(Borders::NONE)
//             .style(Style::default().bg(Color::DarkGray));

//         let exit_text = Text::styled(
//             "Would you like to output the buffers as json? (y/n)",
//             Style::default().fg(Color::Red),
//         );

//         let exit_paragraph = Paragraph::new(exit_text)
//             .block(popup_block)
//             .wrap(Wrap { trim: false });

//         let area = centered_rect(60, 25, frame.area());
//         frame.render_widget(exit_paragraph, area);
//     };
// }
