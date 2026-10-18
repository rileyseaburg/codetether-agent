//! Swarm run state, selection and shared runtime control.
use crate::{SubTaskInfo, SwarmEvent};
use codetether_swarm_core::SwarmStats;
use ratatui::widgets::ListState;
use tokio::sync::mpsc;

/// State for the swarm view.
///
/// # Examples
/// ```
/// let state = codetether_swarm_view::SwarmViewState::new();
/// assert!(!state.active);
/// assert_eq!(state.progress(), 0.0);
/// ```
#[derive(Debug, Default)]
pub struct SwarmViewState {
    /// Whether swarm mode is active
    pub active: bool,
    /// The main task being executed
    pub task: String,
    /// All subtasks
    pub subtasks: Vec<SubTaskInfo>,
    /// Current stage (0-indexed)
    pub current_stage: usize,
    /// Total stages
    pub total_stages: usize,
    /// Stats from execution
    pub stats: Option<SwarmStats>,
    /// Any error message
    pub error: Option<String>,
    /// Whether execution is complete
    pub complete: bool,
    /// Currently selected subtask index
    pub selected_index: usize,
    /// Whether we're in detail mode (viewing a single agent)
    pub detail_mode: bool,
    /// Scroll offset within the detail view
    pub detail_scroll: usize,
    /// ListState for StatefulWidget rendering
    pub list_state: ListState,
    /// Optional event receiver for live swarm updates
    pub(super) event_rx: Option<mpsc::Receiver<SwarmEvent>>,
    /// Runtime control handle for the active swarm (cancel/pause/resume).
    pub control: Option<codetether_session_control::ControlHandle>,
}
