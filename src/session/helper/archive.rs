//! S3/R2 archival of session event streams.
//!
//! Sessions that run with `CODETETHER_EVENT_STREAM_PATH` set append
//! structured JSONL events to a per-session directory. When S3/R2 credentials
//! are configured, those files are archived after each prompt for immutable
//! compliance logging (SOC 2, FedRAMP, ATO).

use std::path::PathBuf;

use crate::event_stream::s3_sink::S3Sink;

/// Resolve the event-stream base directory from the
/// `CODETETHER_EVENT_STREAM_PATH` environment variable.
pub(crate) fn event_stream_path() -> Option<PathBuf> {
    std::env::var("CODETETHER_EVENT_STREAM_PATH")
        .ok()
        .map(PathBuf::from)
}

/// Spawn a background task that uploads every `*.jsonl` event file under the
/// given session's event-stream directory to S3/R2.
///
/// This is best-effort: missing configuration, a missing directory, or an S3
/// sink construction failure are all treated as no-ops so they never block
/// the prompt from returning.
pub(crate) async fn archive_event_stream_to_s3(session_id: &str, base_dir: Option<PathBuf>) {
    if !S3Sink::is_configured() {
        return;
    }

    let Some(base_dir) = base_dir else {
        return;
    };

    let session_event_dir = base_dir.join(session_id);
    if !session_event_dir.exists() {
        return;
    }

    let Ok(sink) = S3Sink::from_env().await else {
        tracing::warn!("Failed to create S3 sink for archival");
        return;
    };

    let session_id = session_id.to_string();
    tokio::spawn(async move {
        let Ok(mut entries) = tokio::fs::read_dir(&session_event_dir).await else {
            return;
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.extension().is_none_or(|e| e != "jsonl") {
                continue;
            }
            match sink.upload_file(&path, &session_id).await {
                Ok(url) => tracing::info!(url = %url, "Archived event stream to S3/R2"),
                Err(e) => tracing::warn!(error = %e, "Failed to archive event file to S3"),
            }
        }
    });
}
