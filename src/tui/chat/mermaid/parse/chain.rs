//! Split a chained flowchart statement into individual edge segments.

use crate::tui::chat::mermaid::parse::arrow::{split_arrow, take_label};

/// One `from --arrow--> to` hop within a statement.
pub struct Segment<'a> {
    /// Source node token, still carrying any bracketed label.
    pub from: &'a str,
    /// The matched arrow token.
    pub arrow: &'static str,
    /// Optional `|label|` attached to this hop.
    pub label: Option<String>,
    /// Target node token.
    pub to: &'a str,
}

/// Expand `A --> B --> C` into one segment per arrow.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::chain::segments;
///
/// let segs = segments("A[One] --> B --> C");
/// assert_eq!(segs.len(), 2);
/// assert_eq!(segs[1].from.trim(), "B");
/// assert_eq!(segs[1].to.trim(), "C");
/// ```
pub fn segments(line: &str) -> Vec<Segment<'_>> {
    let mut out = Vec::new();
    let mut cur = line;
    while let Some((from, arrow, rest)) = split_arrow(cur) {
        let (label, tail) = take_label(rest);
        let to = split_arrow(tail).map_or(tail, |(node, _, _)| node);
        out.push(Segment {
            from,
            arrow,
            label,
            to,
        });
        cur = tail;
    }
    out
}
