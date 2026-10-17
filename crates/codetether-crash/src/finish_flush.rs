//! Persist the existing backoff outcome and emit flush diagnostics.
use super::*;
pub(super) async fn finish_flush(settings: &CrashReporterSettings, attempt: FlushAttempt) {
    let FlushAttempt {
        sent,
        failed,
        status_failures: _,
        transport_error,
    } = attempt;
    // Update persistent backoff state.
    // - If we sent at least one report, reset failures.
    // - If we sent none and had transport failure or repeated rejections, increase backoff.
    if sent > 0 {
        let _ = save_upload_state(
            CrashUploadState {
                consecutive_failures: 0,
                next_attempt_at: None,
                last_attempt_at: Some(Utc::now()),
                last_error: None,
            },
            settings,
        )
        .await;
    } else if failed > 0 {
        let err_str = transport_error
            .as_ref()
            .map(|e| truncate_with_ellipsis(&format!("{e}"), 300));

        let _ = bump_upload_backoff(err_str, settings).await;

        if let Some(err) = transport_error {
            tracing::warn!(
                endpoint = %settings.endpoint,
                error = %err,
                "Crash report upload failed (transport). Backing off and will retry later."
            );
        }
    }

    tracing::info!(
        sent = sent,
        failed = failed,
        endpoint = %settings.endpoint,
        "Crash report sync complete"
    );
}
