//! Agent-tree panel selection with frontend-provided row composition.
use crate::view::Host;
use codetether_agent_tree::AgentTreeState;
use ratatui::text::Line;

/// Read-only adapter for the dashboard and selected agent's detail panel.
///
/// The session type remains opaque to this adapter. Row callbacks let the
/// frontend select its existing transcript and worker projections.
///
/// # Examples
/// ```
/// use codetether_agent_dashboard::{panel::Panel, view::Host};
/// let tree = codetether_agent_tree::AgentTreeState::<()>::default();
/// let panel = Panel {
///     tree: &tree, dashboard: Vec::new, detail: Vec::new,
/// };
/// assert!(!panel.detail_mode());
/// assert!(panel.selected().is_none());
/// ```
pub struct Panel<'a, S, D, F> {
    /// Agent registry and panel selection state.
    pub tree: &'a AgentTreeState<S>,
    /// Compose rows for the overview panel.
    pub dashboard: D,
    /// Compose rows for the selected agent's detail panel.
    pub detail: F,
}

impl<S, D, F> Host for Panel<'_, S, D, F>
where
    D: Fn() -> Vec<Line<'static>>,
    F: Fn() -> Vec<Line<'static>>,
{
    fn detail_mode(&self) -> bool {
        self.tree.detail_mode
    }
    fn selected(&self) -> Option<&str> {
        self.tree.active.as_deref()
    }
    fn scroll(&self) -> u16 {
        self.tree.detail_scroll
    }
    fn dashboard_lines(&self) -> Vec<Line<'static>> {
        (self.dashboard)()
    }
    fn detail_lines(&self) -> Vec<Line<'static>> {
        (self.detail)()
    }
}
