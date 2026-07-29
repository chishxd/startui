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
    "Music PLayer only on Linux for now :( Sorry for that".to_string
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| {
        loop {
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

                let center_pane = Block::default().borders(Borders::ALL).title("Center Pane");

                frame.render_widget(left_pane, cols[0]);
                frame.render_widget(center_pane, cols[1]);

                let center_panes = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(10), Constraint::Min(3)])
                    .split(cols[1]);

                let song_widget = Paragraph::new(get_current_song())
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
        }
    })
}
