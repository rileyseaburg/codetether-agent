//! Ordered session execution and task settlement for streamed requests.
use super::*;

/// Execute one stream task through application session operations.
///
/// # Arguments
///
/// Supply the task table, optional context id, task id, prompt and session host.
///
/// # Returns
///
/// Returns after execution and task settlement, or after a recorded setup error.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo<H: codetether_a2a_http::task_state::stream::driver::StreamSessionHost>(
/// # host: &mut H, tasks: &dashmap::DashMap<String, codetether_a2a_core::Task>) {
/// codetether_a2a_http::task_state::stream::driver::run(
///     tasks, None, &"task-1".to_string(), "hello", host,
/// ).await;
/// # }
/// ```
pub async fn run<H: StreamSessionHost>(
    tasks: &DashMap<String, Task>,
    context_id: Option<String>,
    task_id: &String,
    prompt: &str,
    host: &mut H,
) {
    let started_at = Instant::now();

    // Create a channel for session events
    let (event_tx, event_rx) = mpsc::channel::<H::Event>(256);

    let mut session = match host.resolve(context_id.as_deref()).await {
        Ok(s) => s,
        Err(e) => {
            start_failure::record(tasks, task_id, prompt, started_at, e);
            return;
        }
    };
    host.configure(&mut session).await;

    host.spawn_events(event_rx);

    let Some(registry) = host.registry().await else {
        return;
    };

    match host.execute(&mut session, prompt, event_tx, registry).await {
        Ok(result_text) => {
            host.persist(&mut session).await;
            complete::complete(tasks, task_id, context_id, prompt, started_at, result_text);
        }
        Err(e) => {
            failure::record(tasks, task_id, prompt, started_at, e);
        }
    }
}
