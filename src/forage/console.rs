//! Console output gating for forage runs.
//!
//! Forage is launched both from the CLI (where human-readable cycle output
//! to stdout is desired) and from the TUI (where raw `println!` corrupts the
//! ratatui screen). The TUI renders its own summary via the update channel,
//! so it sets quiet mode to suppress direct stdout writes.

use std::sync::atomic::{AtomicBool, Ordering};

static QUIET: AtomicBool = AtomicBool::new(false);

/// Suppress (or re-enable) direct stdout writes from forage cycles.
///
/// The TUI sets this to `true` before launching a run so cycle output does
/// not bleed onto the terminal UI.
pub fn set_quiet(quiet: bool) {
    QUIET.store(quiet, Ordering::Relaxed);
}

/// Whether forage should skip direct stdout writes.
pub fn is_quiet() -> bool {
    QUIET.load(Ordering::Relaxed)
}

/// Print a line to stdout unless forage is running in quiet mode.
///
/// Use this in place of `println!` for human-facing cycle output so the TUI
/// can suppress it.
#[macro_export]
macro_rules! forage_println {
    ($($arg:tt)*) => {
        if !$crate::forage::console::is_quiet() {
            println!($($arg)*);
        }
    };
}

use super::ForageOpportunity;
/// Print a forage cycle's selected opportunities to stdout (quiet-gated).
pub(super) fn print_cycle(cycle: usize, selected: &[ForageOpportunity], show_moonshot: bool) {
    forage_println!("\n=== Forage Cycle {cycle} ===");
    if selected.is_empty() {
        forage_println!("No OKR opportunities found (active/draft/on_hold remaining work).");
        return;
    }
    for (idx, item) in selected.iter().enumerate() {
        item.print_cycle_entry(idx + 1, show_moonshot);
    }
}
