//! Sync I/O helpers for the session listing index.
//!
//! Kept separate from [`super::index_file`] so the async path stays small
//! and the blocking file ops live next to each other.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use anyhow::{Context, Result};
use std::collections::HashMap;

use super::summary::SessionSummary;

/// Append a single summary line to the index. O_APPEND.
pub(super) fn append_sync(path: &Path, summary: &SessionSummary) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("open index {}", path.display()))?;
    let mut line = serde_json::to_vec(summary)?;
    line.push(b'\n');
    file.write_all(&line)?;
    file.flush()?;
    Ok(())
}

/// Read the index from disk, collapsing duplicates so the last line per
/// id wins. Also returns the raw (non-empty) line count so the caller
/// can decide whether the index is bloated enough to compact.
pub(super) fn read_sync(path: &Path) -> Result<(HashMap<String, SessionSummary>, usize)> {
    let file = File::open(path).with_context(|| format!("open index {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut out: HashMap<String, SessionSummary> = HashMap::new();
    let mut total_lines = 0usize;
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        if line.is_empty() {
            continue;
        }
        total_lines += 1;
        if let Ok(summary) = serde_json::from_str::<SessionSummary>(&line) {
            out.insert(summary.id.clone(), summary);
        }
    }
    Ok((out, total_lines))
}
