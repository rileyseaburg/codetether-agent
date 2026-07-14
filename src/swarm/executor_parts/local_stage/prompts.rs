//! System and user prompts for a local sub-agent.

use super::job::Job;
use crate::swarm::tool_policy;

pub(super) fn build(job: &Job) -> (String, String) {
    let directory = job.workspace.display().to_string();
    let system = tool_policy::system_prompt(tool_policy::SystemPromptInput {
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
