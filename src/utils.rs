use chrono::{TimeZone, Utc};
use gray_matter::{engine::YAML, Matter};
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fs, io::Write, path::PathBuf};

/// Converts a JavaScript timestamp to a date string
pub fn convert_timestamp_to_date(timestamp: i64) -> String {
    let seconds = timestamp / 1000;
    let nanoseconds = (timestamp % 1000) * 1_000_000;

    let datetime = Utc.timestamp_opt(seconds, nanoseconds as u32).unwrap();
    let date = datetime.format("%Y-%m-%d").to_string();

    date
}

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    date: i64,
}

/// Generates a hashmap of shows and their corresponding paths with the production date as the key
pub fn generate_hashmap_from_shows() -> HashMap<String, (String, PathBuf)> {
    let matter = Matter::<YAML>::new();
    let files = fs::read_dir("./shows").unwrap();

    let mut shows_map: HashMap<String, (String, PathBuf)> = HashMap::new();
    let mut blocked_dates: Vec<String> = vec![];

    for show in files {
        match show {
            Ok(show) => {
                let path = fs::canonicalize(show.path()).unwrap();
                let file_contents = fs::read_to_string(&path).unwrap();
                let result = matter.parse(&file_contents);
                let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();

                // Converts the JavaScript timestamp to a date string
                let date = convert_timestamp_to_date(front_matter.date);

                if shows_map.contains_key(&date) {
                    // Multiple entries found for `date`, let's remove the entries and blacklist the date to force manual intervention later
                    blocked_dates.push(date.to_string());
                    shows_map.remove(&date);
                }

                if blocked_dates.contains(&date) {
                    continue;
                }

                shows_map.insert(date, (front_matter.title, path));
            }
            Err(e) => {
                panic!("🛑 Error reading shows: {}", e);
            }
        }
    }

    shows_map
}

/// Appends to the frontmatter specifically beneath the url key
pub fn update_frontmatter(path: &PathBuf, key: &str, value: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(path)?;

    let url_regex = Regex::new(r"(?m)^url:.*$")?;

    if let Some(captures) = url_regex.captures(&content) {
        let url = &captures[0];
        let new_content = url_regex.replace(&content, format!("{}\n{}: {}", url, key, value));
        let mut file = fs::File::create(path)?;
        file.write_all(new_content.as_bytes())?;
    }

    Ok(())
}
