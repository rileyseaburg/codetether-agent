//! Bus log storage and ingestion state.

use ratatui::widgets::ListState;

use super::entry::BusLogEntry;

/// Mutable state for the protocol bus log view.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::bus_log::BusLogState;
///
/// let state = BusLogState::new();
/// assert_eq!(state.visible_count(), 0);
/// assert!(state.auto_scroll);
/// ```
#[derive(Debug)]
pub struct BusLogState {
    /// All retained protocol entries.
    pub entries: Vec<BusLogEntry>,
    /// Selected index within the filtered entries.
    pub selected_index: usize,
    /// Whether the detail pane is active.
    pub detail_mode: bool,
    /// Vertical scroll offset for detail mode.
    pub detail_scroll: usize,
    /// Current filter text.
    pub filter: String,
    /// Whether typed input edits the filter.
    pub filter_input_mode: bool,
    /// Whether new entries keep selection pinned to the end.
    pub auto_scroll: bool,
    /// Ratatui list selection state.
    pub list_state: ListState,
    /// Maximum retained entry count.
    pub max_entries: usize,
}

impl Default for BusLogState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            selected_index: 0,
            detail_mode: false,
            detail_scroll: 0,
            filter: String::new(),
            filter_input_mode: false,
            auto_scroll: true,
            list_state: ListState::default(),
            max_entries: 2_000,
        }
    }
}
