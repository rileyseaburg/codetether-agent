//! Transcript messages for recorded approval decisions.

pub(super) fn text(
    action: &super::parse::Action<'_>,
    stored: &super::store::StoredDecision,
) -> String {
    match &stored.tool {
        Some(tool) => format!("{} `{}` for `{tool}`.", action.intent.label(), stored.id),
        None => format!(
            "{} approval request `{}`.",
            action.intent.label(),
            stored.id
        ),
    }
}
