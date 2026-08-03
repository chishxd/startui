use std::{io, process::Command, time::Duration};

use chrono::Local;
use crossterm::event::{self, KeyCode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListState, Paragraph},
};

struct App {
    song_title: String,
    scroll_offset: usize,
    tick_counter: u32,
    rss_state: ListState,
    active_pane: ActivePane,
}

#[derive(PartialEq, Clone, Copy)]
enum ActivePane {
    Left,
    Center,
    Right,
}

impl App {
    fn new() -> Self {
        let mut rss_state = ListState::default();
        rss_state.select(Some(0));

        Self {
            song_title: "No Music Playing".to_string(),
            scroll_offset: 0,
            tick_counter: 0,
            rss_state,
            active_pane: ActivePane::Left,
        }
    }

    fn tick(&mut self) {
        self.tick_counter = self.tick_counter.wrapping_add(1);

        if self.tick_counter.is_multiple_of(4) {
            let new_song = get_current_song();
            if new_song != self.song_title {
                self.scroll_offset = 0;
                self.song_title = new_song;
            }
        }

        self.scroll_offset = self.scroll_offset.wrapping_add(1);
    }

    fn handle_input(&mut self, key: KeyCode) {
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

    fn scroll_rss_up(&mut self) {
        let current = self.rss_state.selected().unwrap_or(0);

        let total_items = 5; //TODO: Change this to actual length of RSS pane

        let next = if current > 0 {
            current - 1
        } else {
            total_items - 1 //Wrap to last item in list
        };

        self.rss_state.select(Some(next));
    }
    fn scroll_rss_down(&mut self) {
        let current = self.rss_state.selected().unwrap_or(0);

        let total_items = 5; //TODO: Change this to actual length of RSS pane

        let next = if current < total_items - 1 {
            current + 1
        } else {
            0 //Wrap to first item in list
        };

        self.rss_state.select(Some(next));
    }
}

fn draw(frame: &mut Frame, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(frame.area());

    draw_left_pane(frame, cols[0], app);
    draw_center_pane(frame, cols[1], app);
    draw_right_pane(frame, cols[2], app);
}

fn draw_left_pane(frame: &mut Frame, area: Rect, app: &mut App) {
    let border_color = if app.active_pane == ActivePane::Left {
        Color::Yellow
    } else {
        Color::DarkGray
    };
    let left_pane = Block::default()
        .borders(Borders::ALL)
        .title("Left Pane")
        .border_style(Style::default().fg(border_color));
    let items = ["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];

    let list = List::new(items)
        .block(left_pane)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol(">  ");

    frame.render_stateful_widget(list, area, &mut app.rss_state);
}

fn draw_center_pane(frame: &mut Frame, area: Rect, app: &App) {
    let border_color = if app.active_pane == ActivePane::Center {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let center_panes = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(3)])
        .split(area);
    let max_width = (center_panes[0].width as usize).saturating_sub(4);
    let song_chars: Vec<char> = app.song_title.chars().collect();
    let display_song = if song_chars.len() <= max_width {
        app.song_title.clone()
    } else {
        let loop_padding: Vec<char> = "  |  ".chars().collect();
        let mut looped = song_chars.clone();

        looped.extend(&loop_padding);
        looped.extend(&song_chars);

        let start = app.scroll_offset % (song_chars.len() + loop_padding.len());
        let end = start + max_width;
        let window = &looped[start..end];

        let display_str: String = window.iter().collect();
        display_str
    };

    let song_widget = Paragraph::new(display_song)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Music")
                .border_style(Style::default().fg(border_color)),
        )
        .style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(song_widget, center_panes[0]);
}

fn draw_right_pane(frame: &mut Frame, area: Rect, app: &mut App) {
    let border_color = if app.active_pane == ActivePane::Right {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let right_panes = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(3)])
        .split(area);

    let date_str = Local::now().format("%A, %b %d").to_string();
    let time_str = Local::now().format("%I:%M:%S %p").to_string();

    let clock_display = format!("{}\n\n{}", date_str, time_str);

    let clock_widget = Paragraph::new(clock_display)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Clock")
                .border_style(Style::default().fg(border_color)),
        )
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(ratatui::style::Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(clock_widget, right_panes[0]);
    //frame.render_widget(Block::default().borders(Borders::ALL), right_panes[1]);
}

fn main() -> io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| draw(frame, &mut app))?;

            if event::poll(Duration::from_millis(250))?
                && let event::Event::Key(key) = event::read()?
            {
                if key.code == event::KeyCode::Char('q') {
                    break Ok(());
                }

                app.handle_input(key.code);
            }

            app.tick();
        }
    })
}

// I will put helper functions here
//
#[cfg(target_os = "linux")]
fn get_current_song() -> String {
    let output = Command::new("playerctl")
        .args(["metadata", "--format", "{{title}} - {{artist}}"])
        .output();

    match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => "No Music Playing".to_string(),
    }
}

#[cfg(not(target_os = "linux"))]
fn get_current_song() -> String {
    "Music PLayer only on Linux for now :( Sorry for that".to_string()
}
