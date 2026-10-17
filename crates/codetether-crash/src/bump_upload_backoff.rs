//! Crash reporting bump upload backoff.
use super::*;
pub(super) async fn bump_upload_backoff(
    last_error: Option<String>,
    settings: &CrashReporterSettings,
) -> Result<()> {
    let mut state = load_upload_state(settings).await.unwrap_or_default();
    state.consecutive_failures = state.consecutive_failures.saturating_add(1);
    state.last_attempt_at = Some(Utc::now());
    state.last_error = last_error;

    let backoff = compute_backoff_secs(state.consecutive_failures);
    state.next_attempt_at = Some(Utc::now() + chrono::Duration::seconds(backoff as i64));

    save_upload_state(state, settings).await
}
