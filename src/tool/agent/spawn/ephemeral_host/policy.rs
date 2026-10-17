//! Preserve direct-worker intent classification and prompt instructions.
use super::ApplicationHost;
use codetether_agent_input::SpawnRequest;
use codetether_agent_spawn::EphemeralPolicy;
use std::path::Path;
impl EphemeralPolicy for ApplicationHost {
    fn intent(request: &SpawnRequest<'_>) -> (bool, bool) {
        let read_only = crate::tool::swarm_execute::support::is_read_only(
            request.name,
            request.instructions,
            None,
            None,
        );
        let expects_changes = crate::tool::swarm_execute::support::expects_changes(
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
        crate::tool::swarm_execute::agent_prompt::build(
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
