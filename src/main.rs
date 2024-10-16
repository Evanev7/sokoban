#![allow(unused)]

mod app;
mod enums;
mod ui;

use std::io;

use crate::app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut app = App::default();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
