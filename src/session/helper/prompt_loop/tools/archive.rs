//! Fire-and-forget JSONL archival of completed tool calls.

use super::{super::Runner, call::Call, outcome::Outcome};
use crate::event_stream::ChatEvent;
use chrono::Utc;

/// Schedules archival of one completed tool call as a JSONL event.
pub(super) fn write(runner: &Runner<'_>, call: &Call, outcome: &Outcome) {
    let Some(base) = super::super::super::archive::event_stream_path() else {
        return;
    };
    let seq = runner.session.messages.len() as u64;
    let workspace = std::env::var("PWD")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
    let event = ChatEvent::tool_result(
        workspace,
        runner.session.id.clone(),
        &call.name,
        outcome.success,
        outcome.duration_ms,
        &outcome.rendered,
        seq,
    );
    let json = event.to_json();
    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
    let name = format!(
        "{timestamp}-chat-events-{:020}-{:020}.jsonl",
        seq * 10000,
        (seq + 1) * 10000
    );
    let path = base.join(&runner.session.id).join(name);
    tokio::spawn(async move {
        append(path, json).await;
    });
}

async fn append(path: std::path::PathBuf, json: String) {
    use tokio::io::AsyncWriteExt;
    if let Some(parent) = path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    let file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .await;
    if let Ok(mut file) = file {
        let _ = file.write_all(json.as_bytes()).await;
        let _ = file.write_all(b"\n").await;
    }
}
