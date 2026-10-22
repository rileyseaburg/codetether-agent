//! Application headless-session adapter for shared inbound-turn routing.
use super::Application;
use anyhow::Result;
use codetether_a2a_core::Task;
use dashmap::DashMap;

/// Run an inbound turn through the shared live/headless decision.
pub(super) async fn execute_turn<H: Application>(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    context_id: Option<&str>,
    from: &str,
    prompt: &str,
) -> Result<String> {
    let prompt = codetether_a2a_http::inbound_turn::prompt(from, prompt);
    codetether_a2a_http::inbound_turn::execute_turn(
        tasks,
        task_id,
        context_id,
        from,
        &prompt,
        headless::<H>(context_id, &prompt),
    )
    .await
}

async fn headless<H: Application>(context_id: Option<&str>, prompt: &str) -> Result<String> {
    let mut session = H::resolve(context_id).await?;
    H::configure(&mut session).await;
    let result = H::prompt(&mut session, prompt).await?;
    H::persist(&session).await;
    Ok(result.text)
}
