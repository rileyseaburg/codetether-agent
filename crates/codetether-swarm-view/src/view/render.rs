//! Swarm render presentation.
use crate::SwarmViewState;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
/// Render swarm overview or selected-agent activity details.
/// # Arguments
/// * `f` - Target terminal frame.
/// * `state` - Execution snapshot and mutable selection.
/// * `area` - Available screen rectangle.
/// # Returns
/// Unit after rendering the selected view.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// let mut state = codetether_swarm_view::SwarmViewState::new();
/// codetether_swarm_view::render_swarm_view(f, &mut state, area);
/// # }
/// ```
pub fn render_swarm_view(f: &mut Frame, state: &mut SwarmViewState, area: Rect) {
    // If in detail mode, render full-screen detail for selected agent
    if state.detail_mode {
        super::render_agent_detail::render_agent_detail(f, state, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header with task + progress
            Constraint::Length(3), // Stage progress
            Constraint::Min(1),    // Subtask list
            Constraint::Length(3), // Stats
        ])
        .split(area);

    // Header
    super::render_header::render_header(f, state, chunks[0]);

    // Stage progress
    super::render_stage_progress::render_stage_progress(f, state, chunks[1]);

    // Subtask list (stateful for selection)
    super::render_subtask_list::render_subtask_list(f, state, chunks[2]);

    // Stats footer
    super::render_stats::render_stats(f, state, chunks[3]);
}
