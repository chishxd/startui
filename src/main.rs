pub mod app;
pub mod config;
pub mod rss;
pub mod ui;
pub mod utils;

use crossterm::event::{self};
use std::{io, time::Duration};

use crate::{app::App, ui::draw};

fn main() -> io::Result<()> {
    let config_file = config::load_config();

    let rx = rss::spawn_fetcher(config_file.rss_feeds.clone());

    let mut app = App::new();
    ratatui::run(|terminal| {
        loop {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(new_items) => {
                        app.rss_items = new_items;
                        app.rss_error = None;
                        app.rss_state.select(Some(0));
                    }
                    Err(err) => {
                        app.rss_items.clear();
                        app.rss_error = Some(err);
                    }
                }
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
