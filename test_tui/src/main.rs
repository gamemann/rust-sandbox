pub mod app;

use app::App;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    // We need ot install color_eyre hooks and such before anything.
    color_eyre::install()?;

    let terminal = ratatui::init();

    let result = App::new().run(terminal);

    ratatui::restore();
    
    result
}