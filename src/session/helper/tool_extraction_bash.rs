//! Bash command extraction for prose-salvaged tool calls.

use regex::Regex;

/// Pull a shell command out of narrated prose.
///
/// Matches a backticked or quoted command first, then reads text following a
/// `run`/`execute` verb. Returns `None` if nothing command-like is found.
pub fn extract_bash_command(text: &str) -> Option<String> {
    let quoted = Regex::new(r#"[`"]([^`"\n]{2,200})[`"]"#).ok()?;
    if let Some(c) = quoted.captures(text) {
        return Some(c[1].trim().to_string());
    }
    let verb = Regex::new(r"(?i)\b(?:run|running|execute)\s+([^.\n]{2,200})").ok()?;
    verb.captures(text).map(|c| c[1].trim().to_string())
}
