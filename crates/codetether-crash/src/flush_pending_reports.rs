//! Process the pending spool in order, stopping on the existing failure limits.
use super::*;
pub(super) async fn flush_pending_reports(settings: &CrashReporterSettings) -> Result<()> {
    // Persistent backoff: if the endpoint is down / network is unavailable, don't
    // spam the logs by retrying every queued crash report on every startup.
    if should_skip_upload_due_to_backoff(settings).await {
        return Ok(());
    }

    let paths = pending_report_paths(&settings.report_dir)?;
    if paths.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(8))
        .user_agent(format!("codetether/{}", settings.app_version))
        .build()
        .context("build crash reporting HTTP client")?;

    let mut attempt = FlushAttempt::default();
    for path in paths {
        let Some(report) = load_pending_report(&path, &mut attempt).await else {
            continue;
        };
        if !send_pending_report(settings, &client, &path, &report, &mut attempt).await {
            break;
        }
    }
    finish_flush(settings, attempt).await;
    Ok(())
}
