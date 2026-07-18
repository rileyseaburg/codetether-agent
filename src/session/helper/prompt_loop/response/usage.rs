//! Provider telemetry and durable goal usage accounting.

use super::super::Runner;
use crate::provider::CompletionResponse;
use crate::session::SessionEvent;
use std::time::Duration;

pub(super) async fn record(
    runner: &mut Runner<'_>,
    response: &CompletionResponse,
    elapsed: Duration,
) {
    super::super::super::usage_record::record_step_usage(
        &runner.model.provider_name,
        &runner.model.model_id,
        &response.usage,
    );
    account_goal(runner, response).await;
    let Some(tx) = &runner.events else { return };
    let _ = tx
        .send(SessionEvent::UsageReport {
            prompt_tokens: response.usage.prompt_tokens,
            completion_tokens: response.usage.completion_tokens,
            duration_ms: elapsed.as_millis() as u64,
            model: runner.model.model_id.clone(),
        })
        .await;
}

async fn account_goal(runner: &mut Runner<'_>, response: &CompletionResponse) {
    let elapsed = runner.progress.goal_accounted_at.elapsed();
    runner.progress.goal_accounted_at = std::time::Instant::now();
    let tokens = response
        .usage
        .prompt_tokens
        .saturating_add(response.usage.completion_tokens);
    if let Err(error) =
        crate::session::tasks::runtime::record_usage(&runner.session.id, tokens, elapsed).await
    {
        tracing::warn!(error = %error, "Failed to account goal usage");
    }
}
