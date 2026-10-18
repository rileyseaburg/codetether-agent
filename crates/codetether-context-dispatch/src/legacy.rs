//! Shared legacy context-policy execution.
use crate::{Source, input::DispatchInputs};
use anyhow::Result;
use codetether_context_policy::projection::helpers::DerivedContext;

pub(super) async fn run<H: Source, S>(input: DispatchInputs<'_, H, S>) -> Result<DerivedContext> {
    codetether_context_legacy::derive_context(
        input.session,
        input.provider,
        input.model,
        input.system_prompt,
        input.tools,
        input.event_tx,
        input.force_keep_last,
        input.hooks,
    )
    .await
}
