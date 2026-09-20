//! Parse `participant`/`actor` declarations in a sequence diagram.

use crate::tui::chat::mermaid::model::{Node, NodeShape};

/// Parse a `participant X as Label` or `actor X` declaration.
///
/// Returns `None` when the line is not a participant declaration.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::participant::declaration;
///
/// assert_eq!(declaration("participant A as Alice").unwrap().label, "Alice");
/// assert_eq!(declaration("actor Bob").unwrap().id, "Bob");
/// assert!(declaration("A->>B: hi").is_none());
/// ```
pub fn declaration(line: &str) -> Option<Node> {
    let rest = line
        .strip_prefix("participant ")
        .or_else(|| line.strip_prefix("actor "))?;
    let (id, label) = match rest.split_once(" as ") {
        Some((id, label)) => (id.trim(), label.trim()),
        None => (rest.trim(), rest.trim()),
    };
    if id.is_empty() {
        return None;
    }
    Some(named(id, label))
}

/// Build a participant node whose label defaults to its id.
pub fn named(id: &str, label: &str) -> Node {
    Node {
        id: id.to_string(),
        label: label.to_string(),
        shape: NodeShape::Rect,
    }
}
