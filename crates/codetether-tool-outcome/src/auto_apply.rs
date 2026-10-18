//! Preserve pending status and attach errors when automatic confirmation fails.
use serde_json::Value;
use std::collections::HashMap;

/// Resolves a pending confirmation through the configured auto-apply policy.
pub(super) async fn apply(
    tool: &str,
    input: &Value,
    content: String,
    success: bool,
    metadata: Option<HashMap<String, Value>>,
) -> (String, bool, Option<HashMap<String, Value>>, bool) {
    match codetether_edit_confirmation::auto_apply_pending_confirmation(
        tool,
        input,
        metadata.as_ref(),
    )
    .await
    {
        Ok(Some((content, success, metadata))) => (content, success, metadata, false),
        Ok(None) => (content, success, metadata, true),
        Err(error) => (
            format!("{content}\n\nTUI edit auto-apply failed: {error}"),
            false,
            metadata,
            true,
        ),
    }
}
