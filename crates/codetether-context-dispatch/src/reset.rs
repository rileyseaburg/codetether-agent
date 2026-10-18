//! Shared reset context-policy execution.
use crate::{Source, input::DispatchInputs};
use anyhow::Result;
use codetether_context_policy::projection::helpers::DerivedContext;

pub(super) async fn run<H: Source, S>(
    input: DispatchInputs<'_, H, S>,
    threshold_tokens: usize,
) -> Result<DerivedContext> {
    codetether_reset_derivation::derive_reset(
        input.session,
        input.provider,
        input.model,
        input.system_prompt,
        input.tools,
        threshold_tokens,
        input.event_tx,
        input.hooks,
    )
    .await
}
