//! Expand abbreviated slash commands while retaining arguments.
//! Use the exported function to process command input without UI state.

/// Expand abbreviated slash commands while retaining arguments.
///
/// # Arguments
/// * `input` — Command input to inspect.
///
/// # Returns
/// The normalized command and its trimmed arguments.
///
/// # Examples
/// ```
/// assert_eq!(codetether_slash_command::normalize_slash_command("/p events"), "/protocol events");
/// ```
pub fn normalize_slash_command(input: &str) -> String {
    let trimmed = input.trim();
    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("").trim();

    let normalized = match command {
        "/h" | "/?" => "/help".to_string(),
        "/s" => "/sessions".to_string(),
        "/w" => "/swarm".to_string(),
        "/r" => "/ralph".to_string(),
        "/b" | "/buslog" | "/logs" => "/bus".to_string(),
        "/p" => "/protocol".to_string(),
        "/m" => "/model".to_string(),
        "/set" => "/settings".to_string(),
        "/sym" => "/symbols".to_string(),
        other => other.to_string(),
    };

    if args.is_empty() {
        normalized
    } else {
        format!("{normalized} {args}")
    }
}
