//! Character-bounded cognition summaries for heartbeats.

pub(super) fn trim_for_heartbeat(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.trim().to_string();
    }
    let mut trimmed = input.chars().take(max_chars).collect::<String>();
    trimmed.push_str("...");
    trimmed.trim().to_string()
}
