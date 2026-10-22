//! Strict incremental journal replay with bounded least-recently-used caching.
//! Append-only changes replay new bytes; replacement and truncation replay all.
//! Malformed or partial events are errors, never discarded governance.
use anyhow::Result;
use codetether_goal_core::{TaskLog, TaskState};
#[path = "state_cache_entry.rs"]
mod entry;
#[path = "state_cache_replay.rs"]
mod replay;
#[path = "state_cache_store.rs"]
mod store;

/// Fold a journal, replaying only appended events when its identity is unchanged.
/// # Arguments
/// * `log` — Append-only journal to inspect.
/// # Returns
/// Current state, or an empty state for a journal that has not been created.
/// # Errors
/// Returns I/O, malformed-event and incomplete-tail errors without caching them.
/// # Examples
/// ```no_run
/// let log = codetether_goal_core::TaskLog::at("session.tasks.jsonl");
/// let state = codetether_goal_runtime::state_cache::load(&log)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn load(log: &TaskLog) -> Result<TaskState> {
    let cell = store::get(log.path());
    let mut cached = cell.lock();
    let mut file = match std::fs::File::open(log.path()) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            anyhow::ensure!(
                cached.is_none(),
                "previously loaded goal journal is missing"
            );
            return Ok(TaskState::default());
        }
        Err(error) => return Err(error.into()),
    };
    let metadata = file.metadata()?;
    anyhow::ensure!(metadata.is_file(), "goal journal is not a regular file");
    let value = replay::read(&mut file, &metadata, cached.as_ref())?;
    let state = value.state.clone();
    *cached = Some(value);
    Ok(state)
}

#[cfg(test)]
#[path = "state_cache_integrity_tests.rs"]
mod integrity_tests;
#[cfg(test)]
#[path = "state_cache_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "state_cache_transitions_tests.rs"]
mod transitions_tests;
