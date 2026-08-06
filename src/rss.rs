use std::io::Write;
use std::{fs::OpenOptions, sync::mpsc, thread, time::Duration};

#[derive(Clone)]
pub struct RssItem {
    pub title: String,
    pub link: String,
}

fn log_error(msg: &str) {
    if let Some(config_dir) = dirs::config_dir() {
        let log_path = config_dir.join("startui/error.log");
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = writeln!(
                file,
                "[{}],  {}",
                chrono::Local::now().format("%Y-%m-%d  %H:%M:%S"),
                msg
            );
        }
    }
}

pub fn spawn_fetcher(feeds: Vec<String>) -> mpsc::Receiver<Result<Vec<RssItem>, String>> {
    let (tx, rx) = mpsc::channel::<Result<Vec<RssItem>, String>>();

    let config_builder = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(5)));
    let agent: ureq::Agent = config_builder.build().into();

    thread::spawn(move || {
        loop {
            let mut items: Vec<RssItem> = Vec::new();
            let mut error_msg = None;

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
                                let log_msg = format!("HTTP request failed for {}: {:?}", url, err);
                                log_error(&log_msg);
                                error_msg = Some("Network Error: Check error.log".to_string());
                            }
                        }
                    }
                    Err(err) => {
                        let log_msg = format!("HTTP request failed for {}: {:?}", url, err);
                        log_error(&log_msg);
                        error_msg = Some("Network Error: Check error.log".to_string());
                    }
                }
            }
            if !items.is_empty() {
                let _ = tx.send(Ok(items));
            } else if let Some(err) = error_msg {
                let _ = tx.send(Err(err));
            }

            thread::sleep(Duration::from_secs(300));
        }
    });
    rx
}
