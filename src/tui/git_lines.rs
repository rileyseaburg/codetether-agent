//! Compose lines for the TUI git view.

use ratatui::text::Line;

use crate::tui::app::state::git_state::GitViewState;
use crate::tui::git_style::heading;

/// Convert captured git state into renderable lines.
pub fn build_git_lines(state: &GitViewState) -> Vec<Line<'_>> {
    let mut lines: Vec<Line<'_>> = Vec::new();
    push_summary(&mut lines, state);
    push_list(&mut lines, "── Recent Commits ──", &state.log_lines);
    push_diff(&mut lines, state);
    push_list(&mut lines, "── Branches ──", &state.branches);
    lines
}

fn branch_label(state: &GitViewState) -> String {
    if let Some(branch) = state.branch.as_deref() {
        return branch.to_string();
    }

    let has_git_data = !state.branches.is_empty()
        || !state.log_lines.is_empty()
        || !state.diff_stat.trim().is_empty()
        || state.dirty_files > 0;

    if has_git_data {
        "(detached HEAD)".to_string()
    } else {
        "(not a git repo)".to_string()
    }
}

fn push_summary<'a>(lines: &mut Vec<Line<'a>>, state: &GitViewState) {
    let branch = branch_label(state);
    lines.push(Line::from(format!("Branch: {branch}")));
    lines.push(Line::from(format!("Dirty files: {}", state.dirty_files)));
    lines.push(Line::from(format!("Captured: {}", state.captured_at)));
    lines.push(Line::from(""));
}

fn push_list<'a>(lines: &mut Vec<Line<'a>>, title: &'a str, items: &'a [String]) {
    lines.push(heading(title));
    for item in items.iter().take(20) {
        lines.push(Line::from(item.as_str()));
    }
    lines.push(Line::from(""));
}

fn push_diff<'a>(lines: &mut Vec<Line<'a>>, state: &'a GitViewState) {
    lines.push(heading("── Diff Stat ──"));
    for line in state.diff_stat.lines() {
        lines.push(Line::from(line));
    }
    lines.push(Line::from(""));
}
