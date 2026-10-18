//! Bind conflict resolution to the shared smaller-budget agent path.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn execute_conflict_resolution(
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &Path,
        story_id: &str,
        prompt: &str,
        bus: Option<Arc<AgentBus>>,
    ) -> anyhow::Result<String> {
        codetether_ralph_agent::conflict::<super::super::model::host::Native>(
            provider,
            model,
            working_dir,
            story_id,
            prompt,
            bus,
        )
        .await
    }
}
