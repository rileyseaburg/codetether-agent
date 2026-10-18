//! Provider request attempts and delegation accounting.

mod context;
mod derived;
mod failover;
mod recovery;
mod request;

use super::Runner;
use crate::provider::CompletionResponse;
use anyhow::Result;

/// Obtains one successful completion, retrying recoverable failures.
///
/// # Errors
///
/// Returns an unrecoverable provider or context-derivation error.
pub(super) async fn complete(runner: &mut Runner<'_>, step: usize) -> Result<CompletionResponse> {
    codetether_prompt_loop::completion::driver::complete(runner, step).await
}

mod driver_host;
