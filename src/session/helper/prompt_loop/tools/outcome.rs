//! Rendering and classification of raw tool execution output.

use super::{super::Runner, call::Call};
use serde_json::Value;
use std::collections::HashMap;

/// Rendered result and classifications produced by one tool execution.
pub(super) struct Outcome {
    /// Output presented to the model and streaming caller.
    pub rendered: String,
    /// Whether the tool reported success.
    pub success: bool,
    /// Structured metadata returned by the tool.
    pub metadata: Option<HashMap<String, Value>>,
    /// End-to-end execution duration in milliseconds.
    pub duration_ms: u64,
    /// Whether a code-search tool returned no matches.
    pub codesearch_miss: bool,
}

/// Applies confirmation behavior and classifies a raw tool tuple.
pub(super) async fn render(
    runner: &mut Runner<'_>,
    call: &Call,
    input: &Value,
    started: std::time::Instant,
    tuple: super::super::super::tool_policy::ToolTuple,
) -> Outcome {
    let (content, success, metadata) = tuple;
    let pending =
        super::super::super::confirmation::tool_result_requires_confirmation(metadata.as_ref());
    let (content, success, metadata, pending) =
        if pending && crate::tool::auto_apply::auto_apply_enabled(&runner.session.metadata) {
            super::auto_apply::apply(call, input, content, success, metadata).await
        } else {
            (content, success, metadata, pending)
        };
    let rendered = if pending {
        super::super::super::confirmation::pending_confirmation_tool_result_content(
            &call.name, &content,
        )
    } else {
        content
    };
    let (duration_ms, codesearch_miss) = super::outcome_detail::classify(
        runner,
        call,
        started,
        success,
        &rendered,
        metadata.as_ref(),
        pending,
    );
    Outcome {
        rendered,
        success,
        metadata,
        duration_ms,
        codesearch_miss,
    }
}
