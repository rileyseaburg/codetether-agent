//! Detection of easy-go aliases.

/// Recognize go/team aliases without matching longer command prefixes.
/// # Arguments
/// * `input` - Raw command text.
/// # Returns
/// True when the first word is go/team, ignoring case.
/// # Examples
/// ```
/// assert!(codetether_run_command::is_easy_go_command("/GO repair"));
/// assert!(!codetether_run_command::is_easy_go_command("/autochat repair"));
/// ```
pub fn is_easy_go_command(input: &str) -> bool {
    let command = input
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    matches!(command.as_str(), "/go" | "/team")
}
