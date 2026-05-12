//! Compose lines for the TUI git view.

use ratatui::text::Line;

use crate::tui::app::state::git_state::GitViewState;
use crate::tui::git_style::heading;

/// Convert captured git state into renderable lines.
pub fn build_git_lines(state: &GitViewState) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    push_summary(&mut lines, state);
    push_list(&mut lines, "── Recent Commits ──", &state.log_lines);
    push_diff(&mut lines, state);
    push_list(&mut lines, "── Branches ──", &state.branches);
    lines
}

fn push_summary(lines: &mut Vec<Line<'static>>, state: &GitViewState) {
    let branch = state.branch.as_deref().unwrap_or("(detached HEAD)");
    lines.push(Line::from(format!("Branch: {branch}")));
    lines.push(Line::from(format!("Dirty files: {}", state.dirty_files)));
    lines.push(Line::from(format!("Captured: {}", state.captured_at)));
    lines.push(Line::from(""));
}

fn push_list(lines: &mut Vec<Line<'static>>, title: &str, items: &[String]) {
    lines.push(heading(title));
    for item in items.iter().take(20) {
        lines.push(Line::from(item.clone()));
    }
    lines.push(Line::from(""));
}

fn push_diff(lines: &mut Vec<Line<'static>>, state: &GitViewState) {
    lines.push(heading("── Diff Stat ──"));
    for line in state.diff_stat.lines() {
        lines.push(Line::from(line.to_string()));
    }
    lines.push(Line::from(""));
}
