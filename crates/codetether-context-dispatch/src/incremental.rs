//! Shared incremental context-policy execution.
use crate::{Source, input::DispatchInputs};
use anyhow::Result;
use codetether_context_policy::projection::helpers::DerivedContext;

pub(super) async fn run<H: Source, S>(
    input: DispatchInputs<'_, H, S>,
    budget_tokens: usize,
) -> Result<DerivedContext> {
    let budget =
        codetether_context_policy::derivation::input_budget::resolve(input.model, budget_tokens);
    codetether_incremental_derivation::derive_incremental(
        input.session,
        input.system_prompt,
        input.tools,
        budget,
    )
    .await
}
