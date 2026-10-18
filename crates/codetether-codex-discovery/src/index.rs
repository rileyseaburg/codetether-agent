use chrono::{DateTime, Utc};
use codetether_codex_records::CodexSessionIndexEntry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Load the newest indexed title for each session, tolerating malformed rows.
/// # Arguments
/// * `codex_home` — Home containing session_index.jsonl.
/// # Returns
/// Titles by session ID, or an empty map if the index cannot be opened.
/// # Examples
/// ```no_run
/// let _titles = codetether_codex_discovery::load_session_index(std::path::Path::new(".codex"));
/// ```
pub fn load_session_index(codex_home: &Path) -> HashMap<String, String> {
    let file = match File::open(codex_home.join("session_index.jsonl")) {
        Ok(file) => file,
        Err(_) => return HashMap::new(),
    };
    let mut titles = HashMap::<String, (DateTime<Utc>, String)>::new();

    for line in BufReader::new(file).lines().map_while(Result::ok) {
        if line.trim().is_empty() {
            continue;
        }
        let Ok(entry) = serde_json::from_str::<CodexSessionIndexEntry>(&line) else {
            continue;
        };
        match titles.get(&entry.id) {
            Some((updated_at, _)) if updated_at >= &entry.updated_at => {}
            _ => {
                titles.insert(entry.id, (entry.updated_at, entry.thread_name));
            }
        }
    }

    titles
        .into_iter()
        .map(|(id, (_, title))| (id, title))
        .collect()
}
