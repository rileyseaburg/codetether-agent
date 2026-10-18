//! Borrowed stage inputs with no independent resource ownership.
use crate::*;
/// Live fields used to plan, spawn and checkpoint a stage.
/// # Examples
/// ```rust,no_run
/// # use codetether_ralph_stage::{Host, Inputs};
/// # fn example<H: Host>(host: &H) {
/// let original = host.inputs();
/// let view = Inputs::<H::Task> {
///     state: original.state, config: original.config,
///     provider: original.provider, model: original.model,
///     event_tx: original.event_tx, bus: original.bus,
///     registry: original.registry, delegation: original.delegation,
///     store: original.store, run_id: original.run_id,
/// };
/// assert_eq!(view.run_id, original.run_id);
/// # }
/// ```
pub struct Inputs<'a, T: TaskHost> {
    /// Live PRD, iteration and progress state.
    pub state: &'a RalphState,
    /// Existing run configuration.
    pub config: &'a RalphConfig,
    /// Provider cloned once for each stage.
    pub provider: &'a Arc<dyn Provider>,
    /// Selected model cloned once for each stage.
    pub model: &'a String,
    /// Optional nonblocking lifecycle event channel.
    pub event_tx: &'a Option<mpsc::Sender<RalphEvent>>,
    /// Optional shared event bus.
    pub bus: &'a Option<Arc<AgentBus>>,
    /// Optional provider registry for relay tasks.
    pub registry: &'a Option<Arc<T::Registry>>,
    /// Optional provider-selection calibration.
    pub delegation: &'a Option<DelegationState>,
    /// Optional persistent run-state service.
    pub store: Option<&'a Arc<dyn RalphStateStore>>,
    /// Current persistent run identity.
    pub run_id: &'a str,
}
