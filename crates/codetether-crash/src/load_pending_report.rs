//! Read and decode one pending report with the existing discard policy.
use super::*;
pub(super) async fn load_pending_report(
    path: &Path,
    attempt: &mut FlushAttempt,
) -> Option<CrashReport> {
    let raw = match tokio::fs::read_to_string(path).await {
        Ok(content) => content,
        Err(err) => {
            attempt.failed += 1;
            tracing::warn!(path = %path.display(), error = %err, "Failed reading crash report");
            return None;
        }
    };

    let report: CrashReport = match serde_json::from_str(&raw) {
        Ok(report) => report,
        Err(err) => {
            attempt.failed += 1;
            tracing::warn!(
                path = %path.display(),
                error = %err,
                "Invalid crash report format; dropping file"
            );
            let _ = tokio::fs::remove_file(path).await;
            return None;
        }
    };

    Some(report)
}
