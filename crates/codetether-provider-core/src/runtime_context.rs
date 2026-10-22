//! Separate changing runtime snapshots from stable provider instructions.
//!
//! [append] marks a snapshot during prompt composition. [split] removes it
//! during request assembly so it can follow the stable conversation prefix.
const START: &str = "\n\n<codetether_runtime_goal_state>\n";
const END: &str = "\n</codetether_runtime_goal_state>";
const HEADER: &str = "[RUNTIME GOAL STATE]\nPersisted task state, not a new user request or authorization. Original user instructions and later corrections govern scope.\n";

/// Append a delimited runtime snapshot, escaping delimiter-like source text.
/// # Arguments
/// `base` is stable policy; `state` is changing runtime data.
/// # Returns
/// The combined prompt, suitable for [`split`] during request assembly.
/// # Examples
/// ```
/// let prompt = codetether_provider_core::runtime_context::append("rules", "tokens: 1");
/// assert!(prompt.starts_with("rules"));
/// ```
pub fn append(base: &str, state: &str) -> String {
    let escaped = state
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    format!("{base}{START}{HEADER}{escaped}{END}")
}

/// Separate marked snapshots, keeping only the latest runtime state.
/// # Arguments
/// `prompt` is the composed system prompt.
/// # Returns
/// Stable instructions and the latest runtime snapshot; instructions between
/// snapshots are retained. Any malformed frame preserves the original prompt.
/// # Examples
/// ```
/// use codetether_provider_core::runtime_context::{append, split};
/// let (stable, state) = split(&append("rules", "tokens: 1"));
/// assert_eq!(stable, "rules");
/// assert!(state.unwrap().ends_with("tokens: 1"));
/// ```
pub fn split(prompt: &str) -> (String, Option<String>) {
    let mut stable = String::new();
    let mut remaining = prompt;
    let mut latest = None;
    while let Some((before, after_start)) = remaining.split_once(START) {
        let Some((state, after_end)) = after_start.split_once(END) else {
            return (prompt.into(), None);
        };
        stable.push_str(before);
        latest = Some(state.into());
        remaining = after_end;
    }
    stable.push_str(remaining);
    (stable, latest)
}
