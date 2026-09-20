//! Mermaid source parsing.
//!
//! [`parse`] turns the body of a ```` ```mermaid ```` fence into a
//! [`Diagram`]. Parsing is deliberately lenient: unknown statements are
//! skipped so a partially streamed diagram still renders.

use crate::tui::chat::mermaid::model::{Diagram, DiagramKind};

pub mod arrow;
pub mod builder;
pub mod chain;
pub mod header;
pub mod node;
pub mod participant;
pub mod sequence;

#[cfg(test)]
mod tests;

use builder::Builder;

/// Parse mermaid source into a [`Diagram`], or `None` if unsupported.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::parse;
///
/// let d = parse("flowchart TD\n  A[Start] --> B[End]").unwrap();
/// assert_eq!(d.nodes.len(), 2);
/// assert_eq!(d.edges.len(), 1);
/// ```
pub fn parse(source: &str) -> Option<Diagram> {
    let mut kind: Option<DiagramKind> = None;
    let mut builder = Builder::default();

    for raw in source.lines() {
        let line = strip_comment(raw);
        if line.is_empty() {
            continue;
        }
        if kind.is_none() {
            kind = header::parse_header(line);
            continue;
        }
        match kind {
            Some(DiagramKind::Sequence) => sequence::push_sequence(&mut builder, line),
            _ => builder.push_statement(line),
        }
    }

    let kind = kind?;
    if builder.nodes.is_empty() {
        return None;
    }
    Some(Diagram {
        kind,
        nodes: builder.nodes,
        edges: builder.edges,
    })
}

fn strip_comment(raw: &str) -> &str {
    let line = raw.split("%%").next().unwrap_or("").trim();
    line.trim_end_matches(';')
}
