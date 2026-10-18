//! Diff rendering for the `multiedit` tool (an array of edits).
//!
//! `multiedit` arguments carry an `edits` array, each element shaped like an
//! `edit` call (`file`/`path`, `old_string`, `new_string`). This renders each
//! element through the same old→new block style as a single edit.

use ratatui::text::Line;

use super::diff_render::push_edit_diff;

const MAX_EDITS: usize = 8;

/// Render each element of a `multiedit` `edits` array as an old→new diff.
pub(super) fn push_multiedit_diff(out: &mut Vec<Line<'static>>, arguments: &str, w: usize) {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(arguments) else {
        return;
    };
    let Some(edits) = v.get("edits").and_then(|e| e.as_array()) else {
        return;
    };
    for edit in edits.iter().take(MAX_EDITS) {
        let Ok(arg) = serde_json::to_string(edit) else {
            continue;
        };
        push_edit_diff(out, &arg, w);
    }
}
