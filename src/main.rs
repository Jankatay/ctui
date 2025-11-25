// custom
mod crates_io;
mod ui;

use crate::crates_io::*;
use crate::ui::*;

// standard
use std::io;

// crossterm
use crossterm::event::{self, KeyEvent, KeyCode};


fn main() -> io::Result<()> {
    // init terminal and ui
    let mut terminal = ratatui::init();
    let mut ui = Ui::new();

    // while app is running
    while !ui.exit {
        // render the app
        terminal.draw(|frame| ui.render(frame));
        // get input
        if let Some(key) = event::read()?.as_key_press_event() {
            // compute input
            ui.compute_key(&key);
        }
    }

    // restore terminal and return
    ratatui::restore();
    //return result;
    return Ok(());
}
