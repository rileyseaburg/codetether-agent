//! Resolve pending confirmation, render status and classify the final result.
use crate::{Context, Outcome};
use serde_json::Value;

/// Applies confirmation behavior and classifies a raw tool tuple.
/// # Arguments
/// `context` supplies the original input and workspace; `input` is the enriched
/// execution input used for confirmation; `started` measures total duration;
/// `tuple` contains raw tool output, status and metadata.
/// # Returns
/// Rendered output, final status and metadata, timing, and search classification.
/// # Examples
/// ```no_run
/// # async fn example(context: codetether_tool_outcome::Context<'_>) {
/// let result = codetether_tool_outcome::render(
///     context, &serde_json::json!({}), std::time::Instant::now(),
///     ("done".into(), true, None)
/// ).await;
/// assert!(result.success);
/// # }
/// ```
pub async fn render(
    mut context: Context<'_>,
    input: &Value,
    started: std::time::Instant,
    tuple: codetether_tool_reporting::ToolTuple,
) -> Outcome {
    let (content, success, metadata) = tuple;
    let pending =
        codetether_edit_confirmation::tool_result_requires_confirmation(metadata.as_ref());
    let (content, success, metadata, pending) =
        if pending && codetether_edit_confirmation::auto_apply_enabled(context.auto_apply_edits) {
            crate::auto_apply::apply(context.tool, input, content, success, metadata).await
        } else {
            (content, success, metadata, pending)
        };
    let rendered = if pending {
        codetether_edit_confirmation::pending_confirmation_tool_result_content(
            context.tool,
            &content,
        )
    } else {
        content
    };
    let (duration_ms, codesearch_miss) = crate::classify::classify(
        &mut context,
        started,
        success,
        &rendered,
        metadata.as_ref(),
        pending,
    );
    Outcome {
        approval_feedback: None,
        rendered,
        success,
        metadata,
        duration_ms,
        codesearch_miss,
    }
}
