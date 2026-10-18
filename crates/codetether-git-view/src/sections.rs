//! Git summary, bounded lists and diff-stat section text.
use crate::{GitViewState, branch_label::branch_label, style::heading};
use ratatui::text::Line;

pub(crate) fn push_summary<'a>(lines: &mut Vec<Line<'a>>, state: &GitViewState) {
    let branch = branch_label(state);
    lines.push(Line::from(format!("Branch: {branch}")));
    lines.push(Line::from(format!("Dirty files: {}", state.dirty_files)));
    lines.push(Line::from(format!("Captured: {}", state.captured_at)));
    lines.push(Line::from(""));
}

pub(crate) fn push_list<'a>(lines: &mut Vec<Line<'a>>, title: &'a str, items: &'a [String]) {
    lines.push(heading(title));
    for item in items.iter().take(20) {
        lines.push(Line::from(item.as_str()));
    }
    lines.push(Line::from(""));
}

pub(crate) fn push_diff<'a>(lines: &mut Vec<Line<'a>>, state: &'a GitViewState) {
    lines.push(heading("── Diff Stat ──"));
    for line in state.diff_stat.lines() {
        lines.push(Line::from(line));
    }
    lines.push(Line::from(""));
}
