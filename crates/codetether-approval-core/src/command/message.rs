//! Transcript messages for recorded approval decisions.

/// Build the transcript message for a recorded command.
///
/// # Arguments
/// * `action` — Intent whose display label is used.
/// * `stored` — Recorded request and tool metadata.
///
/// # Returns
/// The existing approval transcript wording.
///
/// # Examples
/// ```
/// use codetether_approval_core::command::{Action, StoredDecision, decision_text};
/// let action = Action::parse("/deny request-1").unwrap();
/// let stored = StoredDecision { id: "request-1".into(), tool: None };
/// assert!(decision_text(&action, &stored).contains("Denied"));
/// ```
pub fn text(action: &super::parse::Action<'_>, stored: &super::store::StoredDecision) -> String {
    match &stored.tool {
        Some(tool) => format!("{} `{}` for `{tool}`.", action.intent.label(), stored.id),
        None => format!(
            "{} approval request `{}`.",
            action.intent.label(),
            stored.id
        ),
    }
}
