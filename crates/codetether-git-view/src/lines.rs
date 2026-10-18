//! Assemble captured Git state into ordered display sections.
use crate::{
    GitViewState,
    sections::{push_diff, push_list, push_summary},
};
use ratatui::text::Line;

/// Convert captured git state into renderable lines.
/// # Arguments
/// * `state` - Previously captured Git state.
/// # Returns
/// Ordered summary, commit, diff and branch display lines.
/// # Examples
/// ```no_run
/// let state = codetether_git_view::GitViewState::default();
/// assert!(!codetether_git_view::build_git_lines(&state).is_empty());
/// ```
pub fn build_git_lines(state: &GitViewState) -> Vec<Line<'_>> {
    let mut lines: Vec<Line<'_>> = Vec::new();
    push_summary(&mut lines, state);
    push_list(&mut lines, "── Recent Commits ──", &state.log_lines);
    push_diff(&mut lines, state);
    push_list(&mut lines, "── Branches ──", &state.branches);
    lines
}
