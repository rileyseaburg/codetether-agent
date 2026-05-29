//! Small worker environment and string helpers.

pub(super) fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(default)
}

pub(super) fn trim_for_heartbeat(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.trim().to_string();
    }
    let mut trimmed = input.chars().take(max_chars).collect::<String>();
    trimmed.push_str("...");
    trimmed.trim().to_string()
}
