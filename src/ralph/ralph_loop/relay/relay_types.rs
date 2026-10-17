//! Private state exchanged between relay coordinator phases.
use super::*;
pub(in crate::ralph::ralph_loop) struct PreparedRelayTeam {
    pub(in crate::ralph::ralph_loop) sessions: HashMap<String, Session>,
    pub(in crate::ralph::ralph_loop) ordered_agents: Vec<String>,
    pub(in crate::ralph::ralph_loop) _relay_profiles: Vec<RelayAgentProfile>,
}
pub(in crate::ralph::ralph_loop) struct RelayProgress {
    pub(in crate::ralph::ralph_loop) baton: String,
    pub(in crate::ralph::ralph_loop) previous_normalized: Option<String>,
    pub(in crate::ralph::ralph_loop) convergence_hits: usize,
    pub(in crate::ralph::ralph_loop) turns: usize,
}
pub(in crate::ralph::ralph_loop) struct RelayRoundContext<'a> {
    pub(in crate::ralph::ralph_loop) registry: &'a Arc<ProviderRegistry>,
    pub(in crate::ralph::ralph_loop) team: &'a mut PreparedRelayTeam,
    pub(in crate::ralph::ralph_loop) relay: &'a ProtocolRelayRuntime,
    pub(in crate::ralph::ralph_loop) ralph_tx: &'a Option<mpsc::Sender<RalphEvent>>,
    pub(in crate::ralph::ralph_loop) story_id: &'a str,
    pub(in crate::ralph::ralph_loop) max_rounds: usize,
    pub(in crate::ralph::ralph_loop) prompt: &'a str,
}
