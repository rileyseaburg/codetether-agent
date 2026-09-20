//! Parse a single node token such as `A[Label]`, `B(Label)`, or `C{Label}`.

use crate::tui::chat::mermaid::model::{Node, NodeShape};

/// Split a node token into an id plus a shaped label.
///
/// Tokens without brackets yield a rectangle whose label is the id.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::node::parse_node;
/// use codetether_agent::tui::chat::mermaid::model::NodeShape;
///
/// let n = parse_node("A[Start]").unwrap();
/// assert_eq!(n.id, "A");
/// assert_eq!(n.label, "Start");
/// assert_eq!(n.shape, NodeShape::Rect);
/// assert_eq!(parse_node("B").unwrap().label, "B");
/// ```
pub fn parse_node(token: &str) -> Option<Node> {
    let token = token.trim();
    if token.is_empty() {
        return None;
    }
    for (open, close, shape) in [
        ('[', ']', NodeShape::Rect),
        ('(', ')', NodeShape::Round),
        ('{', '}', NodeShape::Diamond),
    ] {
        if let Some(start) = token.find(open)
            && token.ends_with(close)
        {
            let id = token[..start].trim();
            let label = token[start + 1..token.len() - close.len_utf8()].trim();
            return shaped(id, label, shape);
        }
    }
    shaped(token, token, NodeShape::Rect)
}

fn shaped(id: &str, label: &str, shape: NodeShape) -> Option<Node> {
    if id.is_empty() {
        return None;
    }
    let label = if label.is_empty() { id } else { label };
    Some(Node {
        id: id.to_string(),
        label: label.trim_matches('"').to_string(),
        shape,
    })
}
