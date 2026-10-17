//! Application headless-session adapter for shared inbound-turn routing.
use crate::a2a::types::Task;
use anyhow::Result;
use dashmap::DashMap;

/// Run an inbound turn through the shared live/headless decision.
pub(super) async fn execute_turn(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    context_id: Option<&str>,
    from: &str,
    prompt: &str,
) -> Result<String> {
    codetether_a2a_http::inbound_turn::execute_turn(
        tasks,
        task_id,
        context_id,
        from,
        prompt,
        headless(context_id, prompt),
    )
    .await
}

async fn headless(context_id: Option<&str>, prompt: &str) -> Result<String> {
    let mut session = crate::a2a::session_resolve::resolve_session(context_id).await?;
    crate::a2a::session_config::configure(&mut session).await;
    let result = crate::a2a::prompt_runtime::run(&mut session, prompt).await?;
    crate::a2a::session_config::persist(&session).await;
    Ok(result.text)
}
