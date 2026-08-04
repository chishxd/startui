use chrono::Local;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Padding, Paragraph},
};

use crate::App;
use crate::app::ActivePane;

pub fn draw(frame: &mut Frame, app: &mut App) {
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
    let left_block = Block::default()
        .borders(Borders::ALL)
        .title("  RSS FEED  ")
        .padding(Padding::uniform(1))
        .border_style(Style::default().fg(if app.active_pane == ActivePane::Left {
            Color::Yellow
        } else {
            Color::DarkGray
        }));

    if app.rss_items.is_empty() {
        let loading_widget = Paragraph::new("Loading feed...")
            .alignment(Alignment::Center)
            .block(left_block);

        frame.render_widget(loading_widget, area);
    } else {
        let items: Vec<&str> = app
            .rss_items
            .iter()
            .map(|item| item.title.as_str())
            .collect();

        let list = List::new(items)
            .block(left_block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">  ");

        frame.render_stateful_widget(list, area, &mut app.rss_state);
    }
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
