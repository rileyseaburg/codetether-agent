//! Private state exchanged between relay coordinator phases.
use super::*;
pub(crate) struct PreparedRelayTeam<H: Host> {
    pub(crate) sessions: HashMap<String, H::Session>,
    pub(crate) ordered_agents: Vec<String>,
    pub(crate) _relay_profiles: Vec<RelayAgentProfile>,
}
pub(crate) use codetether_ralph_core::relay::RelayProgress;
pub(crate) struct RelayRoundContext<'a, H: Host> {
    pub(crate) registry: &'a Arc<H::Registry>,
    pub(crate) team: &'a mut PreparedRelayTeam<H>,
    pub(crate) relay: &'a ProtocolRelayRuntime,
    pub(crate) ralph_tx: &'a Option<mpsc::Sender<RalphEvent>>,
    pub(crate) story_id: &'a str,
    pub(crate) max_rounds: usize,
    pub(crate) prompt: &'a str,
}
