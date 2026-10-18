//! Selection and scrolling retained by the protocol registry panel.
use crate::RegistryView;
use std::collections::HashSet;

/// Interactive state for the registered-agent list and details pane.
///
/// # Examples
/// ```
/// let state = codetether_protocol_view::RegistryState::default();
/// assert_eq!(state.selected, 0);
/// assert_eq!(state.scroll, 0);
/// ```
#[derive(Default)]
pub struct RegistryState {
    /// Selected card index, clamped by the renderer to available cards.
    pub selected: usize,
    /// Vertical scroll offset in the details pane.
    pub scroll: usize,
}

impl RegistryState {
    /// Combine panel state with the current registration snapshot.
    /// # Arguments
    /// * `registered_agents` — Names known to the worker bridge.
    /// * `worker_id` — Worker identity used for display-card URLs.
    /// # Returns
    /// A view borrowing the snapshot and copying the panel offsets.
    /// # Examples
    /// ```
    /// let state = codetether_protocol_view::RegistryState::default();
    /// let names = std::collections::HashSet::new();
    /// let worker = None;
    /// let view = state.view(&names, &worker);
    /// assert!(view.registered_agents.is_empty());
    /// assert_eq!(view.selected, state.selected);
    /// ```
    pub fn view<'a>(
        &self,
        registered_agents: &'a HashSet<String>,
        worker_id: &'a Option<String>,
    ) -> RegistryView<'a> {
        RegistryView {
            registered_agents,
            worker_id,
            selected: self.selected,
            scroll: self.scroll,
        }
    }
}
