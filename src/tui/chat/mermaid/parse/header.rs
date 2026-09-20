//! Detect the diagram kind from a mermaid block's first directive line.

use crate::tui::chat::mermaid::model::DiagramKind;

/// Classify a mermaid header line.
///
/// Returns `None` for lines that are not a diagram declaration.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::header::parse_header;
/// use codetether_agent::tui::chat::mermaid::model::DiagramKind;
///
/// assert_eq!(parse_header("flowchart LR"), Some(DiagramKind::FlowchartHorizontal));
/// assert_eq!(parse_header("graph TD"), Some(DiagramKind::FlowchartVertical));
/// assert_eq!(parse_header("sequenceDiagram"), Some(DiagramKind::Sequence));
/// assert_eq!(parse_header("A --> B"), None);
/// ```
pub fn parse_header(line: &str) -> Option<DiagramKind> {
    let line = line.trim();
    if line.starts_with("sequenceDiagram") {
        return Some(DiagramKind::Sequence);
    }
    let rest = line
        .strip_prefix("flowchart")
        .or_else(|| line.strip_prefix("graph"))?;
    let dir = rest.trim().to_ascii_uppercase();
    if dir.starts_with("LR") || dir.starts_with("RL") {
        Some(DiagramKind::FlowchartHorizontal)
    } else {
        Some(DiagramKind::FlowchartVertical)
    }
}
