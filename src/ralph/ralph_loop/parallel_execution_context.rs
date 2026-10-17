//! Borrowed host inputs for one parallel-story model operation.
use super::*;
pub(super) struct ParallelExecutionContext<'a> {
    pub(super) provider: &'a Arc<dyn Provider>,
    pub(super) model: &'a str,
    pub(super) story: &'a UserStory,
    pub(super) working_dir: &'a PathBuf,
    pub(super) ralph_tx: &'a Option<mpsc::Sender<RalphEvent>>,
    pub(super) bridge_tx: &'a Option<mpsc::Sender<SwarmEvent>>,
    pub(super) bus: &'a Option<Arc<AgentBus>>,
    pub(super) registry: &'a Option<Arc<ProviderRegistry>>,
    pub(super) delegation: Option<&'a DelegationState>,
    pub(super) relay_enabled: bool,
    pub(super) relay_max_agents: usize,
    pub(super) relay_max_rounds: usize,
    pub(super) max_steps_per_story: usize,
}
