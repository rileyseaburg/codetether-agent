//! Completion candidates from the command catalog.
//! Call `suggestions` with current composer input; results retain catalog order.

/// Find prefix matches, falling back to substring matches.
///
/// # Arguments
/// * `input` — Command input to inspect.
///
/// # Returns
/// An ordered list of matching command names; empty for non-command input.
///
/// # Examples
/// ```
/// assert!(codetether_slash_command::suggestions("/hel").contains(&"/help".to_string()));
/// ```
pub fn suggestions(input: &str) -> Vec<String> {
    if !input.starts_with('/') {
        return Vec::new();
    }
    let raw_query = input.split_whitespace().next().unwrap_or("");
    let query = crate::normalize_slash_command(raw_query).to_lowercase();
    let mut matches: Vec<&str> = crate::SLASH_COMMANDS
        .iter()
        .filter(|cmd| cmd.starts_with(&query))
        .copied()
        .collect();
    if matches.is_empty() && query.len() > 1 {
        let needle = &query[1..];
        matches = crate::SLASH_COMMANDS
            .iter()
            .filter(|cmd| cmd.contains(needle))
            .copied()
            .collect();
    }
    if matches.is_empty() && query == "/" {
        matches = crate::SLASH_COMMANDS.iter().copied().collect();
    }
    matches.into_iter().map(String::from).collect()
}
