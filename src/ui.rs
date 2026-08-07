use chrono::Local;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Padding, Paragraph, Wrap},
};
use sysinfo::System;

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
        let message = if let Some(ref err) = app.rss_error {
            err.as_str()
        } else {
            "Loading feed..."
        };

        let style = if app.rss_error.is_some() {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let loading_widget = Paragraph::new(message)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(left_block)
            .style(style);

        frame.render_widget(loading_widget, area);
    } else {
        let max_width = (area.width as usize).saturating_sub(4);
        let items: Vec<ListItem> = app
            .rss_items
            .iter()
            .map(|item| {
                let mut wrapped = wrap_text(&item.title, max_width);
                wrapped.push('\n');
                ListItem::new(wrapped)
            })
            .collect();

        let list = List::new(items)
            .block(left_block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">  ");

        frame.render_stateful_widget(list, area, &mut app.rss_state);
    }
}

fn wrap_text(text: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut current_line_len = 0;

    for word in text.split_whitespace() {
        if current_line_len + word.len() + 1 > max_width {
            result.push('\n');
            current_line_len = 0;
        } else if !result.is_empty() && current_line_len > 0 {
            result.push(' ');
            current_line_len += 1;
        }
        result.push_str(word);
        current_line_len += word.len();
    }
    result
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

    //Music Widget Stuff
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

    //System Resource stuff

    let monitor_block = Block::default()
        .borders(Borders::ALL)
        .title(" System ")
        .padding(Padding::new(2, 2, 1, 1))
        .border_style(Style::default().fg(border_color));

    let inner_area = monitor_block.inner(center_panes[1]);

    let sys_panes = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(inner_area);

    let spec_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(12),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(sys_panes[0]);

    let logo_text = "  /\\_/\\\n ( o.o )\n  > ^ <\n [StarTUI]";
    let logo_widget = Paragraph::new(logo_text).style(
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD),
    );

    let os_name = System::name().unwrap_or_else(|| "Linux".to_string());
    let kernel_ver = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "localhost".to_string());

    let uptime_secs = System::uptime();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    let final_uptime = format!("{}h {}m", hours, minutes);

    let specs_lines = vec![
        Line::from(vec![
            Span::styled(
                "OS:      ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(os_name),
        ]),
        Line::from(vec![
            Span::styled(
                "Kernel:  ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(kernel_ver),
        ]),
        Line::from(vec![
            Span::styled(
                "Host:    ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(host_name),
        ]),
        Line::from(vec![
            Span::styled(
                "Uptime:  ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(final_uptime),
        ]),
    ];

    let specs_widget = Paragraph::new(specs_lines);
    frame.render_widget(logo_widget, spec_cols[0]);
    frame.render_widget(specs_widget, spec_cols[2]);

    let cpu_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(12)])
        .split(sys_panes[2]);

    let ram_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(12)])
        .split(sys_panes[4]);

    let cpu_gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .percent(app.cpu_usage as u16)
        .label("");
    let cpu_label = Paragraph::new(format!("CPU: {:.1}%", app.cpu_usage))
        .alignment(Alignment::Right)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    let ram_gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .percent(app.mem_usage as u16)
        .label("");

    let ram_label = Paragraph::new(format!("RAM: {:.1}%", app.mem_usage))
        .alignment(Alignment::Right)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(monitor_block, center_panes[1]);

    frame.render_widget(cpu_gauge, cpu_cols[0]);
    frame.render_widget(cpu_label, cpu_cols[1]);

    frame.render_widget(ram_gauge, ram_cols[0]);
    frame.render_widget(ram_label, ram_cols[1]);
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
