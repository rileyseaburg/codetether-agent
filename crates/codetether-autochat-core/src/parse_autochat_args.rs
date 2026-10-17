//! Shared auto-chat request policy.
use super::*;

/// Parse an optional leading count and borrow the remaining task text.
///
/// # Arguments
/// * `rest` - Command arguments.
/// * `default_agents` - Implicit participant count.
/// * `quick_demo_task` - Borrowed task used for count-only input.
///
/// # Returns
/// The count and task, or `None` for empty input.
///
/// # Examples
/// ```
/// use codetether_autochat_core::parse_autochat_args;
/// assert_eq!(parse_autochat_args("2", 3, "demo"), Some((2, "demo")));
/// ```
pub fn parse_autochat_args<'a>(
    rest: &'a str,
    default_agents: usize,
    quick_demo_task: &'a str,
) -> Option<(usize, &'a str)> {
    let rest = rest.trim();
    if rest.is_empty() {
        return None;
    }

    let mut parts = rest.splitn(2, char::is_whitespace);
    let first = parts.next().unwrap_or("").trim();
    if first.is_empty() {
        return None;
    }

    if let Ok(count) = first.parse::<usize>() {
        let task = parts.next().unwrap_or("").trim();
        if task.is_empty() {
            Some((count, quick_demo_task))
        } else {
            Some((count, task))
        }
    } else {
        Some((default_agents, rest))
    }
}
