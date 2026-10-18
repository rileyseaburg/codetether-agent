//! Merge duplicate IDs using the shared summary precedence rules.
use codetether_session_listing::merge_summary;
use codetether_session_metadata::SessionSummary;
use std::collections::HashMap;
pub(crate) fn merge(merged: &mut HashMap<String, SessionSummary>, codex: Vec<SessionSummary>) {
    for session in codex {
        let summary = match merged.remove(&session.id) {
            Some(native) => merge_summary(native, session),
            None => session,
        };
        merged.insert(summary.id.clone(), summary);
    }
}
