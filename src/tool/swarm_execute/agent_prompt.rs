//! Native session instructions for shared direct-worker prompt composition.
use std::path::Path;
pub(crate) fn build(
    task_id: &str,
    specialty: Option<&str>,
    working_dir: &Path,
    model: &str,
    instruction: &str,
    read_only: bool,
    expects_changes: bool,
) -> String {
    codetether_swarm_tool::prompt::build(
        task_id,
        specialty,
        working_dir,
        model,
        instruction,
        read_only,
        expects_changes,
        crate::session::tasks::turn_instructions::append,
    )
}
