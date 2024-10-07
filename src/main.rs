use std::io;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
        },
    widgets::{Block},
    Frame, Terminal,
}

fn main() -> io::Result<()> {
    en
}