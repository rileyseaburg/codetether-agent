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
    anyhow::ensure!(
        input.header_tokens < budget,
        "context window budget exceeded: pinned source text alone requires {} tokens, input budget is {budget}; explicitly supersede or unpin obsolete constraints",
        input.header_tokens
    );
    codetether_incremental_derivation::derive_incremental(
        &super::header_source::HeaderSource(input.session),
        input.system_prompt,
        input.tools,
        budget - input.header_tokens,
    )
    .await
}
