//! Where an inbound A2A turn actually runs.
//!
//! First choice: the interactive TUI session, when one has opted in via
//! [`crate::a2a::live_inbox`]. Otherwise a headless session resolved from
//! `context_id`, as before. Either way the caller gets the assistant's
//! final text or an error and never has to know which path ran.

use std::time::Duration;

use anyhow::Result;
use dashmap::DashMap;

use crate::a2a::live_inbox;
use crate::a2a::types::Task;

/// How often a live turn refreshes its task while the human's session works,
/// so a polling peer sees progress instead of an idle timeout.
const LIVE_HEARTBEAT: Duration = Duration::from_secs(20);

/// Run `prompt` for task `task_id` and return the assistant's final text.
pub(super) async fn execute_turn(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    context_id: Option<&str>,
    from: &str,
    prompt: &str,
) -> Result<String> {
    if live_inbox::is_attached() {
        return live(tasks, task_id, context_id, from, prompt).await;
    }
    headless(context_id, prompt).await
}

async fn live(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    context_id: Option<&str>,
    from: &str,
    prompt: &str,
) -> Result<String> {
    let mut pending = live_inbox::enqueue(task_id, context_id, from, prompt);
    loop {
        match tokio::time::timeout(LIVE_HEARTBEAT, &mut pending).await {
            Ok(outcome) => {
                return outcome.map_err(|reason| anyhow::anyhow!("interactive session: {reason}"));
            }
            Err(_) => touch(tasks, task_id),
        }
    }
}

/// Bump the task's status timestamp so pollers see the turn is alive.
fn touch(tasks: &DashMap<String, Task>, task_id: &str) {
    if let Some(mut task) = tasks.get_mut(task_id) {
        task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
    }
}

async fn headless(context_id: Option<&str>, prompt: &str) -> Result<String> {
    let mut session = crate::a2a::session_resolve::resolve_session(context_id).await?;
    crate::a2a::session_config::configure(&mut session).await;
    let result = crate::a2a::prompt_runtime::run(&mut session, prompt).await?;
    crate::a2a::session_config::persist(&session).await;
    Ok(result.text)
}
