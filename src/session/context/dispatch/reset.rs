//! Reset host execution.
use super::*;
pub(super) async fn run(
    input: DispatchInputs<'_>,
    threshold_tokens: usize,
) -> Result<DerivedContext> {
    derive_reset(
        input.session,
        input.provider,
        input.model,
        input.system_prompt,
        input.tools,
        threshold_tokens,
        input.event_tx,
    )
    .await
}
