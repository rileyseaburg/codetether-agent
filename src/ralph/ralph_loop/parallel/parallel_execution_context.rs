//! Borrowed host inputs for one parallel-story model operation.
use super::*;
pub(in crate::ralph::ralph_loop) struct ParallelExecutionContext<'a> {
    pub(in crate::ralph::ralph_loop) provider: &'a Arc<dyn Provider>,
    pub(in crate::ralph::ralph_loop) model: &'a str,
    pub(in crate::ralph::ralph_loop) story: &'a UserStory,
    pub(in crate::ralph::ralph_loop) working_dir: &'a PathBuf,
    pub(in crate::ralph::ralph_loop) ralph_tx: &'a Option<mpsc::Sender<RalphEvent>>,
    pub(in crate::ralph::ralph_loop) bridge_tx: &'a Option<mpsc::Sender<SwarmEvent>>,
    pub(in crate::ralph::ralph_loop) bus: &'a Option<Arc<AgentBus>>,
    pub(in crate::ralph::ralph_loop) registry: &'a Option<Arc<ProviderRegistry>>,
    pub(in crate::ralph::ralph_loop) delegation: Option<&'a DelegationState>,
    pub(in crate::ralph::ralph_loop) relay_enabled: bool,
    pub(in crate::ralph::ralph_loop) relay_max_agents: usize,
    pub(in crate::ralph::ralph_loop) relay_max_rounds: usize,
    pub(in crate::ralph::ralph_loop) max_steps_per_story: usize,
}
