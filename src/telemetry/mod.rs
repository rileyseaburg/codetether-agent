//! Process telemetry and host-specific memory-guard startup.
//!
//! Counters and reports share the telemetry crate's process-wide state.

pub use codetether_telemetry::*;
pub mod alloc_guard_config;
mod identity;
pub mod rss_watchdog;
#[cfg(test)]
mod tests;

/// Arm memory guards with the application version and report directory.
///
/// # Arguments
/// * `report_dir` - Shared crash-report spool directory.
/// # Examples
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// codetether_agent::telemetry::install_memory_guards("/tmp/reports".into());
/// # });
/// ```
pub fn install_memory_guards(report_dir: std::path::PathBuf) {
    identity::register();
    codetether_telemetry::install_memory_guards(report_dir);
}
