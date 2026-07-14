//! Workspace tracking and codesearch classification for tool output.

use super::{super::Runner, call::Call};
use serde_json::Value;
use std::collections::HashMap;

/// Tracks touched files and classifies empty code-search output.
pub(super) fn classify(
    runner: &mut Runner<'_>,
    call: &Call,
    started: std::time::Instant,
    success: bool,
    rendered: &str,
    metadata: Option<&HashMap<String, Value>>,
    pending: bool,
) -> (u64, bool) {
    if !pending {
        super::super::super::validation::track_touched_files(
            &mut runner.workspace.touched,
            &runner.workspace.cwd,
            &call.name,
            &call.input,
            metadata,
        );
    }
    let duration_ms = started.elapsed().as_millis() as u64;
    let missed =
        super::super::super::runtime::is_codesearch_no_match_output(&call.name, success, rendered);
    (duration_ms, missed)
}
