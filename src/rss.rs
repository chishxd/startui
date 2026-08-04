use std::{sync::mpsc, thread, time::Duration};

#[derive(Clone)]
pub struct RssItem {
    pub title: String,
    pub link: String,
}

pub fn spawn_fetcher(feeds: Vec<String>) -> mpsc::Receiver<Vec<RssItem>> {
    let (tx, rx) = mpsc::channel::<Vec<RssItem>>();

    let config_builder = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(5)));
    let agent: ureq::Agent = config_builder.build().into();

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
    rx
}
