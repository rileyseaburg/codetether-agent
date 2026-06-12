//! Sync I/O helpers for the session listing index.
//!
//! Kept separate from [`super::index_file`] so the async path stays small
//! and the blocking file ops live next to each other.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use anyhow::{Context, Result};
use std::collections::HashMap;

use super::summary::SessionSummary;

/// Serialises in-process index writers: a save's append must not race a
/// listing's compact-rewrite, or the append can land on the unlinked
/// pre-compaction inode (Unix) or fail the rename (Windows). Writers
/// from *other* processes are not covered — a lost cross-process append
/// is self-healing, because the next listing's disk walk re-discovers
/// the session file and re-appends it.
static WRITE_LOCK: Mutex<()> = Mutex::new(());

/// Take the index writer lock. Only call from the blocking pool.
pub(super) fn write_lock() -> MutexGuard<'static, ()> {
    WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

/// Append a single summary line to the index. O_APPEND.
pub(super) fn append_sync(path: &Path, summary: &SessionSummary) -> Result<()> {
    let _guard = write_lock();
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
