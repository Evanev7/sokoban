#![allow(unused)]

mod app;
mod enums;
mod ui;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut app = App::default();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
