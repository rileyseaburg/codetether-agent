//! Track completed mutations and identify empty code-search output.
use serde_json::Value;
use std::collections::HashMap;

/// Tracks touched files and classifies empty code-search output.
pub(super) fn classify(
    context: &mut crate::Context<'_>,
    started: std::time::Instant,
    success: bool,
    rendered: &str,
    metadata: Option<&HashMap<String, Value>>,
    pending: bool,
) -> (u64, bool) {
    if !pending {
        codetether_validation::track_touched_files(
            context.touched,
            context.cwd,
            context.tool,
            context.input,
            metadata,
        );
    }
    let duration_ms = started.elapsed().as_millis() as u64;
    let missed =
        codetether_prompt_policy::is_codesearch_no_match_output(context.tool, success, rendered);
    (duration_ms, missed)
}
