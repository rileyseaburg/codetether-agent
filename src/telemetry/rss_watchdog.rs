//! Resident-memory watchdog and allocator reclamation coordinator.
//!
//! Samples process RSS, preserves pre-OOM evidence, and asks the system
//! allocator to release unused arenas while memory remains elevated.

mod config;
mod reclaim;
mod report;
#[cfg(test)]
mod report_tests;
mod runner;
mod state;
#[cfg(test)]
mod state_tests;

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

pub use config::{ENV_RSS_CRITICAL_MIB, ENV_RSS_SAMPLE_SECS, ENV_RSS_TRIM_SECS, ENV_RSS_WARN_MIB};

/// Starts the process-wide watchdog using `spool_dir` for crash evidence.
///
/// Repeated calls are harmless; only the first call starts a task.
///
/// # Arguments
///
/// * `spool_dir` - Directory where critical-threshold reports are persisted.
///
/// # Returns
///
/// Returns immediately after scheduling the singleton background task.
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::PathBuf;
///
/// codetether_agent::telemetry::rss_watchdog::spawn(PathBuf::from("/tmp/reports"));
/// ```
pub fn spawn(spool_dir: PathBuf) {
    static STARTED: AtomicBool = AtomicBool::new(false);
    if !STARTED.swap(true, Ordering::SeqCst) {
        tokio::spawn(runner::run(spool_dir, config::Config::load()));
    }
}
