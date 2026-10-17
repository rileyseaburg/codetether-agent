//! Stage task selection and borrowed metadata from the application run.
use super::state::Run;
use codetether_swarm_core::stage_runtime::StageInput;
pub(super) fn read<'a>(run: &'a Run<'_>, stage: usize) -> StageInput<'a> {
    let tasks = run
        .orchestrator
        .subtasks_for_stage(stage)
        .into_iter()
        .cloned()
        .collect();
    StageInput {
        tasks,
        failed: &run.failed,
        provider: run.orchestrator.provider(),
    }
}
