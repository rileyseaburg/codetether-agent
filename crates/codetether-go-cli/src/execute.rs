//! Draft, approve and execute one PRD-backed CLI request.
use crate::{Host, Input};
use codetether_autochat_core::{AUTOCHAT_MAX_AGENTS, model_rotation::ModelRegistry};
use uuid::Uuid;
/// Execute the existing CLI objective-approval and Ralph workflow.
/// # Arguments
/// * `input` - Validated task, model and CLI presentation options.
/// # Returns
/// Unit after denial or completed execution/output.
/// # Errors
/// Returns approval I/O, provider or Ralph execution errors.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_go_cli::Host>(input: codetether_go_cli::Input) -> anyhow::Result<()> where H::Registry: codetether_autochat_core::model_rotation::ModelRegistry {
/// codetether_go_cli::execute::<H>(input).await?;
/// # Ok(()) }
/// ```
pub async fn execute<H: Host>(input: Input) -> anyhow::Result<()>
where
    H::Registry: ModelRegistry,
{
    let max_concurrent = if input.easy_go_requested && !input.explicit_count {
        AUTOCHAT_MAX_AGENTS
    } else {
        input.agent_count
    };
    let okr_id = Uuid::new_v4();
    let registry_for_draft = H::load_registry().await.ok();
    let (okr, draft_note) = crate::draft::prepare(
        &input.task,
        &input.model,
        input.agent_count,
        okr_id,
        &registry_for_draft,
    )
    .await;
    let Some(approved_run) =
        crate::approval::approve(&input, okr_id, &okr, draft_note.as_deref()).await?
    else {
        return Ok(());
    };
    crate::pipeline::run::<H>(input, max_concurrent, okr, approved_run, registry_for_draft).await
}
