use super::model::GitEntryKind;
use super::parse::parse_status;
use super::render_panel;

#[test]
fn parse_groups_porcelain_status() {
    let panel = parse_status(
        "## main...origin/main\n M Cargo.toml\nM  src/lib.rs\n?? notes.md\nUU src/compiler.rs\n",
    );
    assert_eq!(panel.branch, "main...origin/main");
    assert_eq!(panel.entries[0].kind, GitEntryKind::Unstaged);
    assert_eq!(panel.entries[1].kind, GitEntryKind::Staged);
    assert_eq!(panel.entries[2].kind, GitEntryKind::Untracked);
    assert_eq!(panel.entries[3].kind, GitEntryKind::Conflicted);
}

#[test]
fn render_shows_clean_repo() {
    let panel = parse_status("## main...origin/main\n");
    assert_eq!(render_panel(&panel), "git: main...origin/main\nclean\n");
}

#[test]
fn render_orders_groups_by_risk() {
    let panel = parse_status("## feature\n M a\nM  b\n?? c\nUU d\n");
    assert_eq!(
        render_panel(&panel),
        "git: feature\nconflicts\n  UU d\nstaged\n  M  b\nunstaged\n   M a\nuntracked\n  ?? c\n"
    );
}
