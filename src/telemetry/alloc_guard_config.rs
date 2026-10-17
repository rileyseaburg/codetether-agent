//! Application-version adapter for allocation-guard startup.

/// Configure the guard ceiling and report spool for this application.
///
/// # Arguments
/// * `spool_dir` - Directory for allocator crash reports.
/// # Examples
/// ```rust,no_run
/// codetether_agent::telemetry::alloc_guard_config::configure("/tmp/reports".into());
/// ```
pub fn configure(spool_dir: std::path::PathBuf) {
    super::identity::register();
    codetether_telemetry::alloc_guard_config::configure(spool_dir);
}
