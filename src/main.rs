use std::{io, process::Command, time::Duration};

use chrono::Local;
use crossterm::event::{self};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

struct App {
    song_title: String,
    scroll_offset: usize,
    tick_counter: u32,
}

impl App {
    fn new() -> Self {
        Self {
            song_title: "No Music Playing".to_string(),
            scroll_offset: 0,
            tick_counter: 0,
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
}

fn draw(frame: &mut Frame, app: &App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(frame.area());

    draw_left_pane(frame, cols[0]);
    draw_center_pane(frame, cols[1], app);
    draw_right_pane(frame, cols[2]);
}

fn draw_left_pane(frame: &mut Frame, area: Rect) {
    let left_pane = Block::default().borders(Borders::ALL).title("Left Pane");

    frame.render_widget(left_pane, area);
}

fn draw_center_pane(frame: &mut Frame, area: Rect, app: &App) {
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
        .block(Block::default().borders(Borders::ALL).title("Music"))
        .style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(song_widget, center_panes[0]);
}

fn draw_right_pane(frame: &mut Frame, area: Rect) {
    let right_panes = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(3)])
        .split(area);

    let date_str = Local::now().format("%A, %b %d").to_string();
    let time_str = Local::now().format("%I:%M:%S %p").to_string();

    let clock_display = format!("{}\n\n{}", date_str, time_str);

    let clock_widget = Paragraph::new(clock_display)
        .block(Block::default().borders(Borders::ALL).title("Clock"))
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(ratatui::style::Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(clock_widget, right_panes[0]);
    frame.render_widget(Block::default().borders(Borders::ALL), right_panes[1]);
}

fn main() -> io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| draw(frame, &app))?;

            if event::poll(Duration::from_millis(250))?
                && let event::Event::Key(key) = event::read()?
                && key.code == event::KeyCode::Char('q')
            {
                break Ok(());
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
