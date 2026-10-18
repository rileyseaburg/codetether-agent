//! Expand user-friendly agent command aliases.
//! Use the exported function to process command input without UI state.

/// Expand user-friendly agent command aliases.
///
/// # Arguments
/// * `input` — Command input to inspect.
///
/// # Returns
/// The expanded command, or the original input for unknown aliases.
///
/// # Examples
/// ```
/// assert_eq!(codetether_slash_command::normalize_easy_command("/add coder"), "/spawn coder");
/// ```
pub fn normalize_easy_command(input: &str) -> String {
    let trimmed = input.trim();
    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("").trim();

    let normalized = match command {
        "/add" => {
            if args.is_empty() {
                "/spawn".to_string()
            } else {
                format!("/spawn {args}")
            }
        }
        "/talk" | "/say" => {
            if args.is_empty() {
                "/agent".to_string()
            } else {
                format!("/agent {args}")
            }
        }
        "/list" | "/ls" => "/agents".to_string(),
        "/remove" | "/rm" => {
            if args.is_empty() {
                "/undo".to_string()
            } else {
                format!("/kill {args}")
            }
        }
        "/focus" => {
            if args.is_empty() {
                "/agent".to_string()
            } else {
                format!("/agent {}", args.trim_start_matches('@'))
            }
        }
        "/home" | "/main" => "/chat".to_string(),
        _ => return input.to_string(),
    };

    normalized
}
