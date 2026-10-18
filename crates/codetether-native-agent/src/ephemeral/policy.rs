//! Preserve direct-worker intent classification and prompt instructions.
use super::{Application, Ephemeral};
use codetether_agent_input::SpawnRequest;
use codetether_agent_spawn::EphemeralPolicy;
use std::path::Path;
impl<T: Application, const MAX_STEPS: usize> EphemeralPolicy for Ephemeral<T, MAX_STEPS> {
    fn intent(request: &SpawnRequest<'_>) -> (bool, bool) {
        let read_only = codetether_swarm_tool::support::is_read_only(
            request.name,
            request.instructions,
            None,
            None,
        );
        let expects_changes = codetether_swarm_tool::support::expects_changes(
            request.name,
            request.instructions,
            None,
            None,
        );
        (read_only, expects_changes)
    }
    fn prompt(
        request: &SpawnRequest<'_>,
        workspace: &Path,
        model: &str,
        read_only: bool,
        expects_changes: bool,
    ) -> String {
        codetether_swarm_tool::prompt::application::build(
            request.name,
            None,
            workspace,
            model,
            request.instructions,
            read_only,
            expects_changes,
        )
    }
}
