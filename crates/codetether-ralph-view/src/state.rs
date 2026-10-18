//! Cached Ralph run state, stories, selection and event receiver.
use crate::{RalphEvent, RalphStoryInfo};
use ratatui::widgets::ListState;
use tokio::sync::mpsc;

// ---------------------------------------------------------------------------
// View state
// ---------------------------------------------------------------------------

/// State for the Ralph view.
///
/// # Examples
/// ```
/// let state = codetether_ralph_view::RalphViewState::new();
/// assert!(!state.active);
/// assert_eq!(state.progress(), 0.0);
/// ```
#[derive(Debug, Default)]
pub struct RalphViewState {
    /// Whether Ralph mode is active
    pub active: bool,
    /// Project name
    pub project: String,
    /// Feature name
    pub feature: String,
    /// All stories
    pub stories: Vec<RalphStoryInfo>,
    /// Current iteration
    pub current_iteration: usize,
    /// Maximum iterations
    pub max_iterations: usize,
    /// Whether execution is complete
    pub complete: bool,
    /// Final status string
    pub final_status: Option<String>,
    /// Any error message
    pub error: Option<String>,
    /// Currently selected story index
    pub selected_index: usize,
    /// Whether we're in detail mode (viewing a single story's agent)
    pub detail_mode: bool,
    /// Scroll offset within the detail view
    pub detail_scroll: usize,
    /// ListState for StatefulWidget rendering
    pub list_state: ListState,
    /// Inbound channel for events from a Ralph loop spawned via `/ralph run`.
    ///
    /// `None` when the view is being used purely as a passive monitor. When
    /// a run is launched from the TUI, the slash-command handler installs a
    /// receiver here and the tick loop drains it into [`Self::handle_event`].
    #[doc(hidden)]
    pub event_rx: Option<mpsc::Receiver<RalphEvent>>,
}
