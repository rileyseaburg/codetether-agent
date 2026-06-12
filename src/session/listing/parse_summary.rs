//! Shared session-file header parser used by both the full-directory
//! rebuild path and the per-file path inside the index-aware walk.
//!
//! Reads only the top-level fields (`id`, `title`, `created_at`,
//! `updated_at`, `metadata.directory`, `agent`, `messages`) — never the
//! rest of the transcript — so listing is cheap even for multi-megabyte
//! sessions.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::record::SessionListingRecord;
use super::summary::SessionSummary;
use super::workspace::matches_workspace;

/// Parse a session file into a [`SessionSummary`], optionally filtering
/// by workspace. Returns `None` for unreadable, malformed, or
/// non-matching files (with a warning logged).
pub(super) fn parse_summary(path: &Path, workspace: Option<&Path>) -> Option<SessionSummary> {
    let file = File::open(path).map_err(log_read(path)).ok()?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    let record: SessionListingRecord = serde_json::from_reader(reader)
        .map_err(log_malformed(path))
        .ok()?;
    let summary = record.into_summary();
    matches_workspace(summary.directory.as_deref(), workspace).then_some(summary)
}

/// Like [`parse_summary`] but does not apply the workspace filter; used
/// when the caller wants to keep a summary for the index regardless of
/// which workspace the listing is scoped to.
pub(super) fn parse_summary_unfiltered(path: &Path) -> Option<SessionSummary> {
    let file = File::open(path).map_err(log_read(path)).ok()?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    let record: SessionListingRecord = serde_json::from_reader(reader)
        .map_err(log_malformed(path))
        .ok()?;
    Some(record.into_summary())
}

fn log_read(path: &Path) -> impl FnOnce(std::io::Error) {
    |error| tracing::warn!(path = %path.display(), error = %error, "skipping unreadable session file")
}

fn log_malformed(path: &Path) -> impl FnOnce(serde_json::Error) {
    |error| tracing::warn!(path = %path.display(), error = %error, "skipping malformed session file")
}
