//! Read helpers for [`FileDurableLog`]: line counting and offset replay.

use crate::bus::BusEnvelope;
use anyhow::Result;
use std::path::Path;

/// Count the lines (= current max offset) in a partition file.
///
/// Returns `0` when the file does not yet exist.
pub(super) async fn count_lines(path: &Path) -> Result<u64> {
    match tokio::fs::read_to_string(path).await {
        Ok(s) => Ok(s.lines().count() as u64),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(0),
        Err(e) => Err(e.into()),
    }
}

/// Replay envelopes whose 1-based offset is strictly greater than `after`.
///
/// Malformed lines are skipped (a corrupt tail must not abort replay), and a
/// missing file replays as empty.
pub(super) async fn read_after(path: &Path, after: u64) -> Result<Vec<BusEnvelope>> {
    let contents = match tokio::fs::read_to_string(path).await {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };
    let out = contents
        .lines()
        .enumerate()
        .filter(|(i, _)| (*i as u64) >= after)
        .filter_map(|(_, l)| serde_json::from_str::<BusEnvelope>(l).ok())
        .collect();
    Ok(out)
}
