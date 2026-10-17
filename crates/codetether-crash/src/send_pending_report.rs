//! Upload one decoded report and decide whether the flush should continue.
use super::*;
pub(super) async fn send_pending_report(
    settings: &CrashReporterSettings,
    client: &reqwest::Client,
    path: &Path,
    report: &CrashReport,
    attempt: &mut FlushAttempt,
) -> bool {
    match upload_report(settings, client, report).await {
        Ok(true) => {
            attempt.sent += 1;
            if let Err(err) = tokio::fs::remove_file(path).await {
                tracing::warn!(
                    path = %path.display(),
                    error = %err,
                    "Failed deleting uploaded crash report"
                );
            }
        }
        Ok(false) => {
            attempt.failed += 1;
            attempt.status_failures += 1;
            if attempt.status_failures >= MAX_STATUS_FAILURES_PER_FLUSH {
                tracing::warn!(
                    failures = attempt.status_failures,
                    endpoint = %settings.endpoint,
                    "Crash report uploads are being rejected; stopping this flush to avoid log spam"
                );
                return false;
            }
        }
        Err(err) => {
            attempt.failed += 1;
            // Transport errors (DNS/TLS/timeout) will repeat for every queued report.
            // Log once and stop trying until the next backoff window.
            attempt.transport_error = Some(err);
            return false;
        }
    }
    true
}
