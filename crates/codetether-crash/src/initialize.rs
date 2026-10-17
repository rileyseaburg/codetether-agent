//! Crash reporting initialize.
use super::*;
/// Install panic capture and memory guards, then flush when consent permits.
///
/// # Arguments
/// * `config` - Application configuration containing crash-report preferences.
/// * `app_version` - Version of the calling application, included in reports.
/// * `install_memory_guards` - Host callback invoked after installing the panic hook.
/// # Returns
/// Completes after setup and the optional flush; failures are logged.
/// # Examples
/// ```no_run
/// # async fn example(config: &codetether_config::Config) {
/// codetether_crash::initialize(config, "application-version", codetether_telemetry::install_memory_guards).await;
/// # }
/// ```
pub async fn initialize(
    config: &Config,
    app_version: &str,
    install_memory_guards: impl FnOnce(PathBuf),
) {
    let settings = CrashReporterSettings::from_config(config, app_version);
    install_panic_hook(settings.clone());

    // Arm the memory guards next to the panic hook: the capacity-guarding
    // allocator (clean abort + spooled report on a runaway allocation) and
    // the RSS watchdog (pre-OOM breadcrumbs). Both write into the same spool
    // the crash-report flusher already drains on next startup, and both are
    // idempotent, so calling `initialize` twice is harmless.
    install_memory_guards(settings.report_dir.clone());

    if !settings.enabled {
        return;
    }

    if settings.auth_token.is_none() && settings.api_key.is_none() {
        tracing::info!(
            "Crash reporting enabled without auth headers. Set CODETETHER_CRASH_REPORT_AUTH_TOKEN or CODETETHER_CRASH_REPORT_API_KEY if your endpoint requires authentication."
        );
    }

    if let Err(err) = flush_pending_reports(&settings).await {
        tracing::warn!(error = %err, "Failed to flush pending crash reports");
    }
}
