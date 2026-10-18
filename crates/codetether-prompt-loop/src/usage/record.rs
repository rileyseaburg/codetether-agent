use super::Host;
use codetether_provider_core::CompletionResponse;
use codetether_session_events::SessionEvent;
use std::time::Duration;
/// Record provider metrics, goal consumption and the live usage report.
/// # Arguments
/// `host` supplies native services; `response` supplies token counts;
/// `elapsed` is the duration of this completion step.
/// # Returns
/// Completes after accounting and optional event delivery. Storage failures
/// are logged, and a closed event channel is ignored.
/// # Examples
/// ```no_run
/// # async fn report(host: &mut impl codetether_prompt_loop::usage::Host, response: &codetether_provider_core::CompletionResponse) {
/// codetether_prompt_loop::usage::record(host, response, std::time::Duration::from_secs(1)).await;
/// # }
/// ```
pub async fn record(host: &mut impl Host, response: &CompletionResponse, elapsed: Duration) {
    let (provider, model) = host.target();
    codetether_provider_metrics::record_step_usage(provider, model, &response.usage);
    super::account::account_goal(host, response).await;
    let Some(tx) = host.events() else { return };
    let _ = tx
        .send(SessionEvent::UsageReport {
            prompt_tokens: response.usage.prompt_tokens,
            completion_tokens: response.usage.completion_tokens,
            duration_ms: elapsed.as_millis() as u64,
            model: host.target().1.to_owned(),
        })
        .await;
}
