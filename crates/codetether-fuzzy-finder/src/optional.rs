//! Fuzzy finder optional presentation.
use crate::FuzzyFindState;
use crate::render::render;
use ratatui::{Frame, layout::Rect};

/// Render the fuzzy finder if active. No-op otherwise.
/// # Arguments
/// * `f` - Target frame.
/// * `area` - Available screen rectangle.
/// * `state` - Active state, or None when closed.
/// # Returns
/// Unit; draws only when a state is present.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// codetether_fuzzy_finder::render_if_active(f, area, &None);
/// # }
/// ```
pub fn render_if_active(f: &mut Frame, area: Rect, state: &Option<FuzzyFindState>) {
    if let Some(state) = state {
        render(f, area, state);
    }
}
