//! Preserve the legacy sub-query context length policy.
use super::super::truncate_with_ellipsis;
pub(super) fn subquery_context(context: &str) -> String {
    let context_chars = context.chars().count();
    if context_chars > 8000 {
        format!(
            "{}\n[truncated, {} chars total]",
            truncate_with_ellipsis(context, 7500),
            context_chars
        )
    } else {
        context.to_string()
    }
}
