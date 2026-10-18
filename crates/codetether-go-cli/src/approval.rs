//! CLI approval gate and approved-run persistence.
use crate::Input;
use codetether_okr::{Okr, OkrRun};
use codetether_relay_plan::truncate_with_ellipsis;
use uuid::Uuid;
pub(super) async fn approve(
    input: &Input,
    okr_id: Uuid,
    okr: &Okr,
    draft_note: Option<&str>,
) -> anyhow::Result<Option<OkrRun>> {
    let mut run = codetether_relay_okr::cli_approval::new_run(okr_id);

    // Show OKR draft
    if !codetether_relay_okr::cli_approval::ask(
        &mut run,
        codetether_relay_okr::cli_approval::Prompt {
            easy_go_requested: input.easy_go_requested,
            task_preview: &truncate_with_ellipsis(&input.task, 80),
            agent_count: input.agent_count,
            model: &input.model,
            draft_note,
            okr,
        },
    )? {
        return Ok(None);
    }

    // Save OKR and run
    let mut approved_run = run;
    codetether_relay_okr::cli_approval::persist(okr_id, okr, &mut approved_run).await;

    Ok(Some(approved_run))
}
