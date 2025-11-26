// the tui of the app.

// crossterm
use crossterm::event::{KeyEvent, KeyCode};
// ratatui
use ratatui::{
    widgets::{Paragraph, Block, Borders, Wrap},
    prelude::Constraint::{Ratio, Max, Fill, Percentage},
    style::{Style, Modifier, Color},
    prelude::{Layout, Position},
    text::{Text, Line},
    layout::Rect,
    Frame
};

// misc
use crates_io_api::*;

pub struct App {
    pub searchbar: String,
    pub selected_crate: String,
    pub editing: bool,
    pub exit: bool
}

impl App {
    // default info at startup. "Search bar will be empty" etc.
    pub fn new() -> Self {
        Self{
            searchbar: String::with_capacity(32),
            selected_crate: String::from("hello, world"),
            editing: false,
            exit: false
        }
    }

    // compute how each frame should be drawn.
    pub fn render(&self, frame: &mut Frame) {
        // designate areas
        let [search_area, res_area, desc_area] = triplet_layout(frame.area());

        // render the widgets
        frame.render_widget(self.search_widget(), search_area);
        frame.render_widget(self.desc_widget(), desc_area);

        // set cursor position
        let mut cursor_x = self.searchbar.len();
        if(cursor_x > 17) { 
            cursor_x = 17; 
        }
        frame.set_cursor_position(Position::new(1 + search_area.x + (cursor_x as u16), search_area.y+1));
    } 

    // compute how to handle some key
    pub fn compute_key(&mut self, key: &KeyEvent) {
        match key.code {
            // quit on <esc>
            KeyCode::Esc => self.exit = true,
            // send key-inputs to searchbar
            KeyCode::Char(c) => self.searchbar.push(c),
            // backspace removes last search
            KeyCode::Backspace => _ = self.searchbar.pop(),
            // tab switches mode
            KeyCode::Tab => self.editing = !self.editing,
            // ignore rest
            _ => {}
        }
    }

    // turn the text bar into the widget
    fn search_widget(&self) -> Paragraph {
        // Make it orange while editing.
        let mut search_style = Style::default();
        if self.editing {
            search_style = search_style.fg(Color::Yellow);
        }
        
        // setup the borders
        let search_borders = Block::new()
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM);

        // setup the substring to show on searchbar
        let mut search_index = 0;
        if self.searchbar.len() >= 18 {
            search_index = self.searchbar.len() - 18;
        } 
        // setup and return the widget
        return Paragraph::new(&self.searchbar[search_index..])
            .style(search_style).block(search_borders)
            .wrap(Wrap{trim: true});
    }

    // description of the selected crate.
    fn desc_widget(&self) -> Paragraph {
        // Make it yellow while not editing search-bar.
        let mut desc_style = Style::default();
        if !self.editing {
            desc_style = desc_style.fg(Color::Yellow);
        }
        
        // setup the borders
        let desc_borders = Block::bordered();

        // setup and return the widget
        return Paragraph::new(self.selected_crate.as_str())
            .style(desc_style).block(desc_borders)
            .wrap(Wrap{trim: true});
    }
}

// app's layout
// ----------------------
// |      | ........... |
// |======| ........... |
// |      | ........... |
// |      | ........... |
// |      | ........... |
// |      | ........... |
// ----------------------
fn triplet_layout(area: Rect) -> [Rect; 3] {
    // left and right half ratios
    let ratios_horz = Layout::horizontal([Max(20), Fill(1)]);

    // search bar to results space ratios
    let ratios_vert = Layout::vertical([Max(3), Percentage(0)]);

    // split the rayouts 
    let [crate_area, desc_area] = ratios_horz.areas(area);
    let [search_area, res_area] = ratios_vert.areas(crate_area);

    // return the three areas.
    return [search_area, res_area, desc_area];
}

