//! Workspace filter + sort helper for listings.

use std::path::Path;

use super::summary::SessionSummary;
use super::workspace::matches_workspace;

/// Apply workspace filter, then sort by `updated_at` desc.
pub(super) fn filter_and_sort(
    summaries: Vec<SessionSummary>,
    workspace: Option<&Path>,
) -> Vec<SessionSummary> {
    let mut out: Vec<SessionSummary> = summaries
        .into_iter()
        .filter(|s| matches_workspace(s.directory.as_deref(), workspace))
        .collect();
    out.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    out
}
