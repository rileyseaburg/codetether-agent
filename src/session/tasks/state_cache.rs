//! Memoized task-state folding keyed by log file identity.
//!
//! The TUI event loop checks the goal/answer-review hold after every
//! session event. Re-reading and re-folding an append-only log that grows
//! with every usage update made `/goal` sessions progressively slower and
//! allocation-heavy. This cache re-folds only when the file length or
//! modification time changes; append-only writes always change the length.

use super::{TaskLog, TaskState};
use anyhow::Result;
use parking_lot::Mutex;
use std::{collections::HashMap, path::PathBuf, sync::LazyLock, time::SystemTime};

const MAX_ENTRIES: usize = 64;

type Stamp = (u64, SystemTime);
static CACHE: LazyLock<Mutex<HashMap<PathBuf, (Stamp, TaskState)>>> = LazyLock::new(Mutex::default);

/// Fold the log into state, reusing the last fold when the file is unchanged.
///
/// # Errors
///
/// Returns IO errors from reading an existing but unreadable log.
pub(crate) fn load(log: &TaskLog) -> Result<TaskState> {
    let Ok(meta) = std::fs::metadata(log.path()) else {
        return Ok(TaskState::default());
    };
    let Ok(modified) = meta.modified() else {
        return Ok(TaskState::from_log(&log.read_all_blocking()?));
    };
    let stamp = (meta.len(), modified);
    if let Some((cached, state)) = CACHE.lock().get(log.path())
        && *cached == stamp
    {
        return Ok(state.clone());
    }
    let state = TaskState::from_log(&log.read_all_blocking()?);
    let mut cache = CACHE.lock();
    if cache.len() >= MAX_ENTRIES {
        cache.clear();
    }
    cache.insert(log.path().to_path_buf(), (stamp, state.clone()));
    Ok(state)
}

#[cfg(test)]
#[path = "state_cache_tests.rs"]
mod tests;
