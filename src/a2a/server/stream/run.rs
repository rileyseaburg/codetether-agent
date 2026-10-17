//! Resolve a session, start observers and execute one prompt.
use super::*;
pub(super) async fn run(
    tasks: Arc<DashMap<String, Task>>,
    context_id: Option<String>,
    spawn_task_id: String,
    bus: Option<Arc<crate::bus::AgentBus>>,
    prompt: String,
) {
    let task_id = spawn_task_id;
    let started_at = Instant::now();

    // Create a channel for session events
    let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(256);

    let mut session =
        match crate::a2a::session_resolve::resolve_session(context_id.as_deref()).await {
            Ok(s) => s,
            Err(e) => {
                start_failure::record(&tasks, &task_id, &prompt, started_at, e);
                return;
            }
        };
    crate::a2a::session_config::configure(&mut session).await;

    events::spawn(&bus, &tasks, &task_id, event_rx, event_data::data);

    let Some(registry) = registry::load(&tasks, &task_id).await else {
        return;
    };

    match session
        .prompt_with_events(&prompt, event_tx, registry)
        .await
    {
        Ok(result) => {
            let result_text = result.text;
            crate::a2a::session_config::persist(&session).await;
            complete::complete(
                &tasks,
                &task_id,
                context_id,
                &prompt,
                started_at,
                result_text,
            );
        }
        Err(e) => {
            failure::record(&tasks, &task_id, &prompt, started_at, e);
        }
    }
}
