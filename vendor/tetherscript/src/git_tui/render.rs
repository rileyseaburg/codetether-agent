//! Text rendering for the git panel.

use super::model::{GitEntryKind, GitPanel};

/// Render a stable text snapshot of the git panel.
pub fn render_panel(panel: &GitPanel) -> String {
    let mut out = format!("git: {}\n", panel.branch);
    if panel.is_clean() {
        out.push_str("clean\n");
        return out;
    }
    for kind in [
        GitEntryKind::Conflicted,
        GitEntryKind::Staged,
        GitEntryKind::Unstaged,
        GitEntryKind::Untracked,
    ] {
        render_group(panel, kind, label(kind), &mut out);
    }
    out
}

fn render_group(panel: &GitPanel, kind: GitEntryKind, label: &str, out: &mut String) {
    let entries: Vec<_> = panel
        .entries
        .iter()
        .filter(|entry| entry.kind == kind)
        .collect();
    if entries.is_empty() {
        return;
    }
    out.push_str(label);
    out.push('\n');
    for entry in entries {
        out.push_str(&format!("  {} {}\n", entry.code, entry.path));
    }
}

fn label(kind: GitEntryKind) -> &'static str {
    match kind {
        GitEntryKind::Conflicted => "conflicts",
        GitEntryKind::Staged => "staged",
        GitEntryKind::Unstaged => "unstaged",
        GitEntryKind::Untracked => "untracked",
    }
}
