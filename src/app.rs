use crossterm::event::KeyCode;
use ratatui::widgets::ListState;

use crate::{rss::RssItem, utils};

pub struct App {
    pub song_title: String,
    pub scroll_offset: usize,
    pub tick_counter: u32,
    pub rss_state: ListState,
    pub active_pane: ActivePane,
    pub rss_items: Vec<RssItem>,
    pub rss_error: Option<String>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ActivePane {
    Left,
    Center,
    Right,
}

impl Default for App {
    fn default() -> Self {
        let mut rss_state = ListState::default();
        rss_state.select(Some(0));

        Self {
            song_title: "No Music Playing".to_string(),
            scroll_offset: 0,
            tick_counter: 0,
            rss_state,
            active_pane: ActivePane::Left,
            rss_items: Vec::new(),
            rss_error: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        self.tick_counter = self.tick_counter.wrapping_add(1);

        if self.tick_counter.is_multiple_of(4) {
            let new_song = utils::get_current_song();
            if new_song != self.song_title {
                self.scroll_offset = 0;
                self.song_title = new_song;
            }
        }

        self.scroll_offset = self.scroll_offset.wrapping_add(1);
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('h') | KeyCode::Left => {
                self.active_pane = match self.active_pane {
                    ActivePane::Right => ActivePane::Center,
                    ActivePane::Center => ActivePane::Left,
                    ActivePane::Left => ActivePane::Right,
                }
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.active_pane = match self.active_pane {
                    ActivePane::Left => ActivePane::Center,
                    ActivePane::Center => ActivePane::Right,
                    ActivePane::Right => ActivePane::Left,
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if let ActivePane::Left = self.active_pane {
                    self.scroll_rss_up();
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if let ActivePane::Left = self.active_pane {
                    self.scroll_rss_down();
                }
            }
            _ => {}
        }
    }

    pub fn scroll_rss_up(&mut self) {
        if self.rss_items.is_empty() {
            return;
        }

        let current = self.rss_state.selected().unwrap_or(0);

        let next = if current > 0 {
            current - 1
        } else {
            self.rss_items.len() - 1 //Wrap to last item in list
        };

        self.rss_state.select(Some(next));
    }
    pub fn scroll_rss_down(&mut self) {
        if self.rss_items.is_empty() {
            return;
        }
        let current = self.rss_state.selected().unwrap_or(0);

        let next = if current < self.rss_items.len() - 1 {
            current + 1
        } else {
            0 //Wrap to first item in list
        };

        self.rss_state.select(Some(next));
    }
}
