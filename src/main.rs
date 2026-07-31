use std::{io, process::Command, time::Duration};

use chrono::Local;
use crossterm::event::{self};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

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

fn main() -> io::Result<()> {
    ratatui::run(|terminal| {
        let mut last_song = "No Music Playing".to_string();
        let mut scroll_offset: usize = 0;
        let mut tick_counter: u32 = 0;
        loop {
            if tick_counter % 4 == 0 {
                let new_song = get_current_song();
                if new_song != last_song {
                    scroll_offset = 0;
                    last_song = new_song;
                }

                tick_counter = tick_counter.wrapping_add(1);
            }
            terminal.draw(|frame| {
                let cols = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                    ])
                    .split(frame.area());

                let left_pane = Block::default().borders(Borders::ALL).title("Left Pane");

                // let center_pane = Block::default().borders(Borders::ALL).title("Center Pane");

                frame.render_widget(left_pane, cols[0]);
                // frame.render_widget(center_pane, cols[1]);

                let center_panes = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(5), Constraint::Min(3)])
                    .split(cols[1]);

                let max_width = (center_panes[0].width as usize).saturating_sub(4);
                let song_chars: Vec<char> = last_song.chars().collect();

                let display_song = if song_chars.len() <= max_width {
                    last_song.clone()
                } else {
                    let loop_padding: Vec<char> = "  |  ".chars().collect();
                    let mut looped = song_chars.clone();

                    looped.extend(&loop_padding);
                    looped.extend(&song_chars);

                    let start = scroll_offset % (song_chars.len() + loop_padding.len());
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

                let right_panes = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(5), Constraint::Min(3)])
                    .split(cols[2]);

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
            })?;

            if event::poll(Duration::from_millis(250))?
                && let event::Event::Key(key) = event::read()?
                && key.code == event::KeyCode::Char('q')
            {
                break Ok(());
            }
            scroll_offset = scroll_offset.wrapping_add(1);
        }
    })
}
