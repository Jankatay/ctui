// the tui of the app.

// crossterm
use crossterm::event::{KeyEvent, KeyCode};
// ratatui
use ratatui::{
    prelude::{Layout, Constraint::Ratio, Position},
    widgets::{Paragraph, Block, Borders, Wrap},
    style::{Style, Modifier, Color},
    text::{Text, Line},
    Frame
};

// misc
use crates_io_api::*;

pub struct Ui {
    pub searchbar: String,
    pub editing: bool,
    pub exit: bool
}

impl Ui {
    // default info at startup. "Search bar will be empty" etc.
    pub fn new() -> Self {
        Self{
            searchbar: String::with_capacity(32),
            editing: false,
            exit: false
        }
    }

    // compute how each frame should be drawn.
    pub fn render(&self, frame: &mut Frame) {
        // app's layout
        // ----------------------
        // |====| ............. |
        // |----| ............. |
        // |----| ............. |
        // |----| ............. |
        // |----| ............. |
        // |----| ............. |
        // ----------------------
        let layout = Layout::horizontal([
            Ratio(1, 5),    // left
            Ratio(4, 5)     // right
        ]);

        let [search_area, desc_area] = layout.areas(frame.area());

        // setup the search-bar
        let mut search_style = Style::default();
        if self.editing {
            search_style = search_style.fg(Color::Yellow);
        }
        
        let search_borders = Block::new()
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM);

        let search_widget = Paragraph::new(self.searchbar.as_str())
            .style(search_style).block(search_borders)
            .wrap(Wrap{trim: true});

        // render it
        frame.render_widget(search_widget, search_area);
        frame.set_cursor_position(Position::new(search_area.x, search_area.y));
    } 

    // compute how to handle some key
    pub fn compute_key(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Esc => self.exit = true,
            _ => self.searchbar.push('a')
        }
    }
}

