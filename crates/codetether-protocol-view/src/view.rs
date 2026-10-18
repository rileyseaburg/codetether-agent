//! Data borrowed from the host application for registry presentation.
use std::collections::HashSet;
/// Inputs for the registered-agent view.
/// # Examples
/// ```
/// use codetether_protocol_view::RegistryView;
/// let names = std::collections::HashSet::new();
/// let worker = None;
/// let view = RegistryView { registered_agents: &names, worker_id: &worker, selected: 0, scroll: 0 };
/// assert!(view.registered_agents.is_empty());
/// ```
pub struct RegistryView<'a> {
    /// Registered names; displayed in sorted order.
    pub registered_agents: &'a HashSet<String>,
    /// Worker identity used as the display card URL.
    pub worker_id: &'a Option<String>,
    /// Selected card index; clamped to the available cards.
    pub selected: usize,
    /// Vertical detail-pane scroll offset.
    pub scroll: usize,
}
