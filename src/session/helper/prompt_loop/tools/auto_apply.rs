//! Automatic application of tool results requiring confirmation.

use super::call::Call;
use serde_json::Value;
use std::collections::HashMap;

/// Resolves a pending confirmation through the configured auto-apply policy.
pub(super) async fn apply(
    call: &Call,
    input: &Value,
    content: String,
    success: bool,
    metadata: Option<HashMap<String, Value>>,
) -> (String, bool, Option<HashMap<String, Value>>, bool) {
    match super::super::super::confirmation::auto_apply_pending_confirmation(
        &call.name,
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
