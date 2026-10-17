//! Relay context extract context delta from text.
use super::*;

/// Extract a context delta from raw JSON or a semantic final payload.
///
/// # Arguments
/// * `text` - Candidate output from context distillation.
///
/// # Returns
/// A delta when either supported parsing path succeeds.
///
/// # Examples
/// ```
/// use codetether_autochat_core::shared_context::extract_context_delta_from_text;
/// assert!(extract_context_delta_from_text("{}").is_some());
/// ```
pub fn extract_context_delta_from_text(text: &str) -> Option<ContextDelta> {
    if let Some(delta) = parse_json_payload::<ContextDelta>(text) {
        return Some(delta);
    }

    if let FinalPayload::Semantic(payload) = FinalPayload::parse(text)
        && let Some(delta) = parse_json_payload::<ContextDelta>(&payload.answer)
    {
        return Some(delta);
    }

    None
}
