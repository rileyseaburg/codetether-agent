//! Application-version adapter for the process memory watchdog.

pub use codetether_telemetry::rss_watchdog::{
    ENV_RSS_CRITICAL_MIB, ENV_RSS_SAMPLE_SECS, ENV_RSS_TRIM_SECS, ENV_RSS_WARN_MIB,
};

/// Start the singleton watchdog with this application's report identity.
///
/// # Arguments
/// * `spool_dir` - Directory for memory-pressure reports.
/// # Examples
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// codetether_native_telemetry::rss_watchdog::spawn("/tmp/reports".into());
/// # });
/// ```
pub fn spawn(spool_dir: std::path::PathBuf) {
    super::identity::register();
    codetether_telemetry::rss_watchdog::spawn(spool_dir);
}
