//! Resolve the model budget and execute incremental derivation.
use super::*;
pub(super) async fn run(input: DispatchInputs<'_>, budget_tokens: usize) -> Result<DerivedContext> {
    let budget = super::super::input_budget::resolve(input.model, budget_tokens);
    derive_incremental(
        input.session,
        input.provider,
        input.model,
        input.system_prompt,
        input.tools,
        budget,
        input.event_tx,
    )
    .await
}
