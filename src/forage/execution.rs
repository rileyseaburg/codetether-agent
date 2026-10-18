//! Bind forage evidence collection and engine dispatch to application services.
use super::*;
pub(super) async fn execute_opportunity(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<ExecutionOutcome> {
    codetether_forage_execution::execute_opportunity::<super::execution_host::Native>(item, args)
        .await
}
