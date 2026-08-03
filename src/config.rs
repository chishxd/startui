use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub rss_feeds: Vec<String>
}
