//! Goal token and elapsed-time accounting.
use super::Host;
use codetether_provider_core::CompletionResponse;
pub(super) async fn account_goal(host: &mut impl Host, response: &CompletionResponse) {
    let elapsed = host.progress().goal_accounted_at.elapsed();
    host.progress().goal_accounted_at = std::time::Instant::now();
    let tokens = response
        .usage
        .prompt_tokens
        .saturating_add(response.usage.completion_tokens);
    if let Err(error) = host.account(tokens, elapsed).await {
        tracing::warn!(error = %error, "Failed to account goal usage");
    }
}
