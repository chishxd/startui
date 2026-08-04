pub mod config;

use chrono::Local;
use config::Config;
use crossterm::event::{self, KeyCode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListState, Padding, Paragraph},
};
use std::{fs, io, path::PathBuf, process::Command, sync::mpsc, thread, time::Duration};

#[derive(Clone)]
struct RssItem {
    title: String,
    link: String,
}
struct App {
    song_title: String,
    scroll_offset: usize,
    tick_counter: u32,
    rss_state: ListState,
    active_pane: ActivePane,
    rss_items: Vec<RssItem>,
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
            rss_items: Vec::new(),
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
    fn scroll_rss_down(&mut self) {
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

fn main() -> io::Result<()> {
    let config_file = load_config();
    let feeds = config_file.rss_feeds.clone();

    let config_builder = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(5)));
    let agent: ureq::Agent = config_builder.build().into();

    let (tx, rx) = mpsc::channel::<Vec<RssItem>>();

    thread::spawn(move || {
        loop {
            let mut items: Vec<RssItem> = Vec::new();

            for url in &feeds {
                let request = agent
                    .get(url)
                    .header("User-Agent", "StarTUI/1.0 (contact: github.com/chishxd)");

                match request.call() {
                    Ok(response) => {
                        let reader = response.into_body().into_reader();
                        match feed_rs::parser::parse(reader) {
                            Ok(feed) => {
                                for entry in feed.entries.iter().take(15) {
                                    let title = entry
                                        .title
                                        .as_ref()
                                        .map(|t| t.content.clone())
                                        .unwrap_or_else(|| "No Title".to_string());

                                    let link = entry
                                        .links
                                        .first()
                                        .map(|l| l.href.clone())
                                        .unwrap_or_default();

                                    items.push(RssItem { title, link });
                                }
                            }
                            Err(err) => {
                                eprintln!("[RSS ERROR] HTTP request failed for {}: {:?}", url, err);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("[RSS ERROR] HTTP request failed for {}: {:?}", url, err);
                    }
                }
            }
            if !items.is_empty() {
                let _ = tx.send(items);
            }

            thread::sleep(Duration::from_secs(300));
        }
    });
    let mut app = App::new();
    ratatui::run(|terminal| {
        loop {
            if let Ok(new_items) = rx.try_recv() {
                app.rss_items = new_items;
                app.rss_state.select(Some(0));
            }
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

fn load_config() -> Config {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("startui");

    let config_path = config_dir.join("config.toml");

    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);

        let default_toml = r#"# StarTUI Config File
            rss_feeds = [
                "https://news.ycombinator.com/rss",
                "https://reddit.com/r/rust/.rss"
            ]
        "#;

        let _ = fs::write(&config_path, default_toml);
    }

    let config_content = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        eprintln!("[ERROR] Failed to read config file: {}", e);
        std::process::exit(1);
    });

    match toml::from_str(&config_content) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("[CONFIG ERROR] Failed to parse config.toml: {}", err);
            std::process::exit(1);
        }
    }
}
