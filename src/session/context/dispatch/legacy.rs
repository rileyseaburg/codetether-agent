//! Legacy and forced-recovery host execution.
use super::*;
pub(super) async fn run(input: DispatchInputs<'_>) -> Result<DerivedContext> {
    derive_context(
        input.session,
        input.provider,
        input.model,
        input.system_prompt,
        input.tools,
        input.event_tx,
        input.force_keep_last,
    )
    .await
}
