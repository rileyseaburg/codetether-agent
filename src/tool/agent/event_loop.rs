//! Event loop — collects session events from a running agent task.

use super::helpers::truncate_preview;
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use serde_json::{Value, json};
use tokio::sync::mpsc;

pub(super) async fn run(
    rx: &mut mpsc::Receiver<SessionEvent>,
    handle: tokio::task::JoinHandle<Result<Session>>,
) -> (String, String, Vec<Value>, Option<String>, Option<Session>) {
    let mut response = String::new();
    let mut thinking = String::new();
    let mut tools = Vec::new();
    let mut error = None;
    let mut done = false;
    let start = std::time::Instant::now();
    let max_wait = std::time::Duration::from_secs(300);

    while !done && start.elapsed() < max_wait {
        tokio::task::yield_now().await;
        match tokio::time::timeout(std::time::Duration::from_millis(20), rx.recv()).await {
            Ok(Some(event)) => match event {
                SessionEvent::TextComplete(t) => response.push_str(&t),
                SessionEvent::ThinkingComplete(t) => thinking.push_str(&t),
                SessionEvent::ToolCallComplete { name, output, success, duration_ms: _ } => {
                    tools.push(json!({
                        "tool": name,
                        "success": success,
                        "output_preview": truncate_preview(&output, 200),
                    }));
                }
                SessionEvent::Error(e) => {
                    response.push_str(&format!("\n[Error: {e}]"));
                    error = Some(e);
                }
                SessionEvent::Done => done = true,
                _ => {}
            },
            Ok(None) | Err(_) => {
                if handle.is_finished() {
                    done = true;
                }
            }
        }
    }

    let mut updated_session: Option<Session> = None;
    if !handle.is_finished() {
        handle.abort();
        error = Some("Agent timed out after 5 minutes".into());
    } else {
        match handle.await {
            Ok(Ok(session)) => updated_session = Some(session),
            Ok(Err(e)) => error = Some(e.to_string()),
            Err(e) => error = Some(e.to_string()),
        }
    }

    (response, thinking, tools, error, updated_session)
}
