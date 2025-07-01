use feed_rs::{model::Feed, parser};
use reqwest::blocking;
use std::io::Cursor;

pub fn fetch_feed(url: &str) -> Result<Feed, Box<dyn std::error::Error>> {
    let response = blocking::get(url)?;
    let bytes = response.bytes()?;
    let feed = parser::parse(Cursor::new(bytes))?;
    Ok(feed)
}

pub fn print_feed(feed: &Feed) {
    if let Some(title) = &feed.title {
        println!("# {}", title.content);
    }
    for entry in &feed.entries {
        if let Some(entry_title) = &entry.title {
            println!("- {}", entry_title.content);
        }
    }
}

pub fn process_urls_file(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        println!("Fetching {}...", line);
        let feed = fetch_feed(line)?;
        print_feed(&feed);
    }
    Ok(())
}
