//! Construct and schedule one tool-result archive event.
use super::{Record, append::append};
use crate::ChatEvent;
use chrono::Utc;
/// Schedule best-effort JSONL archival when an archive path is configured.
/// # Arguments
/// * `record` — Session, sequence and tool outcome fields.
/// # Returns
/// Returns after spawning the append task; file errors remain best-effort.
/// # Panics
/// Panics outside a Tokio runtime when archival is configured.
/// # Examples
/// ```no_run
/// # async fn example() {
/// codetether_event_stream::tool_archive::write(codetether_event_stream::tool_archive::Record { session_id: "session", message_count: 1, tool_name: "read", success: true, duration_ms: 2, rendered: "contents" });
/// # }
/// ```
pub fn write(record: Record<'_>) {
    let Some(base) = crate::event_stream_path() else {
        return;
    };
    let seq = record.message_count;
    let workspace = std::env::var("PWD")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
    let event = ChatEvent::tool_result(
        workspace,
        record.session_id.to_string(),
        record.tool_name,
        record.success,
        record.duration_ms,
        record.rendered,
        seq,
    );
    let json = event.to_json();
    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
    let name = format!(
        "{timestamp}-chat-events-{:020}-{:020}.jsonl",
        seq * 10000,
        (seq + 1) * 10000
    );
    let path = base.join(record.session_id).join(name);
    tokio::spawn(async move {
        append(path, json).await;
    });
}
