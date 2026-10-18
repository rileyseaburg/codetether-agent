//! Ralph render presentation.
use crate::RalphViewState;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
/// Render the run overview or selected story's detailed activity.
/// # Arguments
/// * `f` - Target terminal frame.
/// * `state` - Run state and mutable story selection.
/// * `area` - Available screen rectangle.
/// # Returns
/// Unit after rendering the overview or detail panel.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// let mut state = codetether_ralph_view::RalphViewState::new();
/// codetether_ralph_view::render_ralph_view(f, &mut state, area);
/// # }
/// ```
pub fn render_ralph_view(f: &mut Frame, state: &mut RalphViewState, area: Rect) {
    if state.detail_mode {
        super::render_story_detail::render_story_detail(f, state, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header: project/feature + iteration
            Constraint::Length(3), // Progress bar
            Constraint::Min(1),    // Story list
            Constraint::Length(3), // Footer / status
        ])
        .split(area);

    super::render_header::render_header(f, state, chunks[0]);
    super::render_progress::render_progress(f, state, chunks[1]);
    super::render_story_list::render_story_list(f, state, chunks[2]);
    super::render_footer::render_footer(f, state, chunks[3]);
}
