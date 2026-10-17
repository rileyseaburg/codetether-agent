//! Execute exactly one selected derivation strategy.
use super::*;
pub(super) async fn run(input: DispatchInputs<'_>, execution: Execution) -> Result<DerivedContext> {
    match execution {
        Execution::Legacy => legacy::run(input).await,
        Execution::Reset { threshold_tokens } => reset::run(input, threshold_tokens).await,
        Execution::Incremental { budget_tokens } => incremental::run(input, budget_tokens).await,
    }
}
