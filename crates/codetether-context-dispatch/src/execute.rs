//! Shared execute context-policy execution.
use crate::{Source, input::DispatchInputs};
use anyhow::Result;
use codetether_context_policy::derivation::dispatch::Execution;
use codetether_context_policy::projection::helpers::DerivedContext;

pub(super) async fn run<H: Source, S>(
    input: DispatchInputs<'_, H, S>,
    execution: Execution,
) -> Result<DerivedContext> {
    match execution {
        Execution::Legacy => super::legacy::run(input).await,
        Execution::Reset { threshold_tokens } => super::reset::run(input, threshold_tokens).await,
        Execution::Incremental { budget_tokens } => {
            super::incremental::run(input, budget_tokens).await
        }
    }
}
