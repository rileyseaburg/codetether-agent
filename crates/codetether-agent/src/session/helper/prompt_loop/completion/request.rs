//! Native adapter for shared completion-attempt request construction.
mod host;
use super::{super::Runner, context::Attempt};
use crate::provider::CompletionRequest;

/// Builds the provider request for the current completion attempt.
pub(super) async fn build(
    runner: &mut Runner<'_>,
    attempt: &mut Attempt,
) -> anyhow::Result<CompletionRequest> {
    codetether_context_request::attempt::build(&mut host::Native { runner, attempt }).await
}
