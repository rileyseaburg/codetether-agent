use crate::browser::{parse_html, Document};
use crate::browser_agent::diff::types::DomDiffSummary;

/// Diffs two HTML strings after parsing them with the built-in browser parser.
///
/// # Arguments
///
/// * `before` - HTML captured before an agent action.
/// * `after` - HTML captured after an agent action.
///
/// # Returns
///
/// A deterministic path-level summary of DOM changes.
pub fn diff_html(before: &str, after: &str) -> DomDiffSummary {
    diff_documents(&parse_html(before), &parse_html(after))
}

/// Diffs two parsed browser documents.
///
/// # Arguments
///
/// * `before` - DOM captured before an agent action.
/// * `after` - DOM captured after an agent action.
///
/// # Returns
///
/// A deterministic path-level summary of DOM changes.
pub fn diff_documents(before: &Document, after: &Document) -> DomDiffSummary {
    let mut entries = Vec::new();
    super::walk::diff_children("", &before.children, &after.children, &mut entries);
    DomDiffSummary { entries }
}
