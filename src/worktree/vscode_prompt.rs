//! Interactive permission prompt for automatic VS Code opening.

use super::WorktreeInfo;
use std::io::{IsTerminal, Write};

/// Ask the user whether to open `info` in VS Code.
///
/// Returns `true` only on an explicit yes. When stdin/stdout is not a TTY
/// (swarm, Ralph, CI), or when an interactive TUI owns the terminal, no raw
/// prompt can be safely shown, so this returns `false` and the caller skips
/// opening rather than corrupting the UI or blocking on a read that never
/// completes.
pub(super) fn confirm_open(info: &WorktreeInfo) -> bool {
    if super::is_tui_active() {
        tracing::info!(worktree = %info.name, "TUI active; skipping VS Code auto-open prompt");
        return false;
    }
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        tracing::info!(worktree = %info.name, "Non-interactive; skipping VS Code auto-open prompt");
        return false;
    }
    print!(
        "📂 Worktree '{}' created at {}.\n   Open it in VS Code now? [y/N]: ",
        info.name,
        info.path.display()
    );
    if std::io::stdout().flush().is_err() {
        return false;
    }
    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    matches!(input.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}
