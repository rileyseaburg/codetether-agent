//! Completion dispatch with whole-turn goal recovery.
use super::Host;
use codetether_provider_core::CompletionResponse;
pub(super) async fn next(
    host: &mut impl Host,
    step: usize,
) -> anyhow::Result<Option<CompletionResponse>> {
    match crate::completion::driver::complete(host, step).await {
        Ok(response) => Ok(Some(response)),
        Err(error) if crate::goal_recovery::after(host, &error).await => Ok(None),
        Err(error) => Err(error),
    }
}
