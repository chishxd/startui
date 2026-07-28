use std::io;

use crossterm::event;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders},
};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                let cols = Layout::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Percentage(50),
                        Constraint::Percentage(20),
                    ])
                    .split(frame.area());

                let left_pane = Block::default().borders(Borders::ALL).title("Left Pane");

                let center_pane = Block::default().borders(Borders::ALL).title("Center Pane");
                let right_pane = Block::default().borders(Borders::ALL).title("Right Pane");

                frame.render_widget(left_pane, cols[0]);
                frame.render_widget(center_pane, cols[1]);
                frame.render_widget(right_pane, cols[2]);
            })?;

            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
