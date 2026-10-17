//! Crash reporting should skip upload due to backoff.
use super::*;
pub(super) async fn should_skip_upload_due_to_backoff(settings: &CrashReporterSettings) -> bool {
    let Some(state) = load_upload_state(settings).await else {
        return false;
    };

    let Some(next) = state.next_attempt_at else {
        return false;
    };

    let now = Utc::now();
    if next > now {
        let secs = (next - now).num_seconds().max(0);
        tracing::info!(
            endpoint = %settings.endpoint,
            retry_in_secs = secs,
            consecutive_failures = state.consecutive_failures,
            "Skipping crash report upload due to backoff"
        );
        return true;
    }

    false
}
