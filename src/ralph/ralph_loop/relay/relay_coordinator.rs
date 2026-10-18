//! Bind story relay execution to the shared orchestration crate.
use super::*;
impl RalphLoop {
    /// Execute a story using a relay team of agents.
    ///
    /// Creates a team of specialist agents (planner, coder, reviewer, etc.),
    /// each with its own Session and full tool access, and runs them in a
    /// baton-passing relay loop. Each agent operates autonomously within its
    /// turn, using tools to read/write code and run commands.
    pub(in crate::ralph::ralph_loop) async fn call_relay_static(
        registry: &Arc<ProviderRegistry>,
        model: &str,
        prompt: &str,
        working_dir: &Path,
        ralph_tx: Option<mpsc::Sender<RalphEvent>>,
        story_id: String,
        bus: Option<Arc<AgentBus>>,
        max_agents: usize,
        max_rounds: usize,
        delegation: Option<&DelegationState>,
    ) -> anyhow::Result<String> {
        codetether_ralph_relay::call_relay::<super::host::Native>(
            registry,
            model,
            prompt,
            working_dir,
            ralph_tx,
            story_id,
            bus,
            max_agents,
            max_rounds,
            delegation,
        )
        .await
    }
}
