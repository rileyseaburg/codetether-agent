//! Native session-instruction binding for shared remote prompt assembly.
use super::*;
pub(super) fn build(payload: &RemoteSubtaskPayload, working_dir: &PathBuf) -> (String, String) {
    codetether_swarm_core::remote_protocol::prompts::build(
        payload,
        working_dir,
        crate::session::tasks::turn_instructions::append,
    )
}
