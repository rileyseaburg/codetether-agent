//! Construct capability-scoped tools for the job workspace.
use super::JobHost;
use codetether_provider_core::ToolDefinition;
use codetether_swarm_local_job::Job;
use std::sync::Arc;
pub(super) fn create<H: JobHost>(job: &Job) -> (Arc<H::Registry>, Vec<ToolDefinition>) {
    let capability = codetether_workspace_tools::Capability::from_task_flags(
        job.read_only,
        job.verification,
        job.expects_changes,
    );
    codetether_workspace_tools::composition::for_agent::<H::Workspace>(
        Arc::clone(&job.provider),
        job.model.clone(),
        &job.workspace,
        capability,
        Arc::clone(&job.result_store),
        job.id.clone(),
    )
}
