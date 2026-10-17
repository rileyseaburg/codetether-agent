//! Format optional handoff context for the relay recipient.
use super::*;
pub(super) fn build(message: String, context: Option<Value>) -> String {
    let context_msg = if let Some(ref ctx) = context {
        format!(
            "{}\n\nContext: {}",
            message,
            serde_json::to_string_pretty(ctx).unwrap_or_default()
        )
    } else {
        message
    };

    context_msg
}
