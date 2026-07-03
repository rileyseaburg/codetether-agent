use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::record::SessionListingRecord;
use super::summary::SessionSummary;

/// Parse a single session file into a full [`SessionSummary`].
///
/// Runs on the blocking pool: serde walks every byte of the `messages` array
/// to count messages, which can be tens of MB per file. Returns `None` for
/// unreadable or malformed files (logged and skipped).
pub(super) fn parse_summary(path: &Path) -> Option<SessionSummary> {
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
