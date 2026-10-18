//! System and user prompts for a local sub-agent.

use super::job::Job;
use codetether_swarm_core::prompt_policy::SystemPromptInput;

pub(super) fn build<H: crate::Host>(job: &Job) -> (String, String) {
    let directory = job.workspace.display().to_string();
    let system = H::system_prompt(SystemPromptInput {
        specialty: &job.specialty,
        subtask_id: &job.id,
        working_dir: &directory,
        model: &job.model,
        instruction: &job.instruction,
        context: &job.context,
        line_limit: None,
        read_only: job.read_only,
        expects_changes: job.expects_changes,
    });
    let user = if job.context.is_empty() {
        format!("Complete this task:\n\n{}", job.instruction)
    } else {
        format!(
            "Complete this task:\n\n{}\n\nContext from prior work:\n{}",
            job.instruction, job.context
        )
    };
    (system, user)
}
