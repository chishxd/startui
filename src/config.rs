use std::{fs, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub rss_feeds: Vec<String>,
    pub bookmarks: Vec<BookmarkConfig>,
}

#[derive(Deserialize, Clone)]
pub struct BookmarkConfig {
    pub name: String,
    pub url: String,
}

pub fn load_config() -> Config {
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
    [[bookmarks]]
     name = "Github"
     url = "https://github.com"

    [[bookmarks]]
     name = "YouTube"
     url = "https://youtube.com"

    [[bookmarks]]
     name = "Reddit"
     url = "https://reddit.com"
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
