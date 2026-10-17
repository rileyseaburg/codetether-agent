//! Private state exchanged between relay coordinator phases.
use super::*;
pub(super) struct PreparedRelayTeam {
    pub(super) sessions: HashMap<String, Session>,
    pub(super) ordered_agents: Vec<String>,
    pub(super) _relay_profiles: Vec<RelayAgentProfile>,
}
pub(super) struct RelayProgress {
    pub(super) baton: String,
    pub(super) previous_normalized: Option<String>,
    pub(super) convergence_hits: usize,
    pub(super) turns: usize,
}
pub(super) struct RelayRoundContext<'a> {
    pub(super) registry: &'a Arc<ProviderRegistry>,
    pub(super) team: &'a mut PreparedRelayTeam,
    pub(super) relay: &'a ProtocolRelayRuntime,
    pub(super) ralph_tx: &'a Option<mpsc::Sender<RalphEvent>>,
    pub(super) story_id: &'a str,
    pub(super) max_rounds: usize,
    pub(super) prompt: &'a str,
}
