//! Parse `sequenceDiagram` message statements such as `A->>B: text`.

use crate::tui::chat::mermaid::model::Edge;
use crate::tui::chat::mermaid::parse::{builder::Builder, participant};

const ARROWS: [&str; 6] = ["-->>", "->>", "-->", "->", "--x", "-x"];

/// Parse one sequence-diagram statement into the builder.
///
/// Handles participant declarations and message arrows; unrecognised
/// statements are ignored so partial diagrams still render.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::builder::Builder;
/// use codetether_agent::tui::chat::mermaid::parse::sequence::push_sequence;
///
/// let mut b = Builder::default();
/// push_sequence(&mut b, "Alice->>Bob: hi");
/// assert_eq!(b.nodes.len(), 2);
/// assert_eq!(b.edges[0].label.as_deref(), Some("hi"));
/// ```
pub fn push_sequence(builder: &mut Builder, line: &str) {
    if let Some(node) = participant::declaration(line) {
        builder.push_node(node);
        return;
    }
    let Some((idx, arrow)) = first_arrow(line) else {
        return;
    };
    let from = line[..idx].trim();
    let (to, label) = split_target(&line[idx + arrow.len()..]);
    if from.is_empty() || to.is_empty() {
        return;
    }
    builder.push_node(participant::named(from, from));
    builder.push_node(participant::named(to, to));
    builder.edges.push(Edge {
        from: from.to_string(),
        to: to.to_string(),
        label,
        dotted: arrow.starts_with("--"),
    });
}

fn split_target(tail: &str) -> (&str, Option<String>) {
    match tail.split_once(':') {
        Some((to, msg)) => (to.trim(), Some(msg.trim().to_string())),
        None => (tail.trim(), None),
    }
}

fn first_arrow(line: &str) -> Option<(usize, &'static str)> {
    ARROWS
        .iter()
        .filter_map(|a| line.find(a).map(|i| (i, *a)))
        .min_by_key(|(i, a)| (*i, std::cmp::Reverse(a.len())))
}
