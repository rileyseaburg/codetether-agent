//! Explicit copy target and command parser.

/// The raw payload kind selected for copying.
/// # Examples
/// ```
/// use codetether_chat_model::copy::{target, Target};
/// assert!(matches!(target("/copy"), Some(Target::Reply)));
/// ```
#[derive(Clone, Copy)]
pub enum Target {
    /// Latest assistant reply.
    Reply,
    /// Latest tool output.
    Tool,
    /// Latest failed tool or error message.
    Error,
}

/// Parse a copy command with at most one explicit target.
/// # Arguments
/// * `prompt` - Complete command text.
/// # Returns
/// The selected target, or None for invalid syntax.
/// # Examples
/// ```
/// assert!(codetether_chat_model::copy::target("/copy tool extra").is_none());
/// ```
pub fn target(prompt: &str) -> Option<Target> {
    let mut words = prompt.split_whitespace();
    if words.next()? != "/copy" {
        return None;
    }
    let target = match words.next() {
        None | Some("reply") => Target::Reply,
        Some("tool") => Target::Tool,
        Some("error") => Target::Error,
        Some(_) => return None,
    };
    words.next().is_none().then_some(target)
}
