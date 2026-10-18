//! Expansion of go/team aliases into relay commands.
use codetether_autochat_core::{AUTOCHAT_DEFAULT_AGENTS, AUTOCHAT_QUICK_DEMO_TASK};

/// Expand go/team aliases to the existing autochat command form.
/// # Arguments
/// * `input` - Raw command text.
/// # Returns
/// A normalized autochat command, or trimmed input for other commands.
/// # Examples
/// ```
/// assert_eq!(codetether_run_command::normalize_cli_go_command("/go 4 repair"), "/autochat 4 repair");
/// ```
pub fn normalize_cli_go_command(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() || !trimmed.starts_with('/') {
        return trimmed.to_string();
    }

    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("").trim();

    match command.to_ascii_lowercase().as_str() {
        "/go" | "/team" => {
            if args.is_empty() {
                format!(
                    "/autochat {} {}",
                    AUTOCHAT_DEFAULT_AGENTS, AUTOCHAT_QUICK_DEMO_TASK
                )
            } else {
                let mut count_and_task = args.splitn(2, char::is_whitespace);
                let first = count_and_task.next().unwrap_or("");
                if let Ok(count) = first.parse::<usize>() {
                    let task = count_and_task.next().unwrap_or("").trim();
                    if task.is_empty() {
                        format!("/autochat {count} {AUTOCHAT_QUICK_DEMO_TASK}")
                    } else {
                        format!("/autochat {count} {task}")
                    }
                } else {
                    format!("/autochat {} {args}", AUTOCHAT_DEFAULT_AGENTS)
                }
            }
        }
        _ => trimmed.to_string(),
    }
}
