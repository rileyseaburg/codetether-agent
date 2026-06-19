//! Per-opportunity cycle line printing for forage CLI output.

use super::ForageOpportunity;
use crate::forage_println;

impl ForageOpportunity {
    /// Print a single ranked opportunity entry (quiet-gated).
    pub(super) fn print_cycle_entry(&self, rank: usize, show_moonshot: bool) {
        forage_println!(
            "\n{}. [{}] {}",
            rank,
            self.okr_status_label(),
            self.okr_title
        );
        forage_println!(
            "   KR: {} ({:.1}% remaining, score {:.3})",
            self.key_result_title,
            self.remaining * 100.0,
            self.score
        );
        forage_println!(
            "   Progress: {:.1}% complete",
            self.progress.clamp(0.0, 1.0) * 100.0
        );
        if show_moonshot {
            let hits = if self.moonshot_hits.is_empty() {
                "none".to_string()
            } else {
                self.moonshot_hits.join(" | ")
            };
            forage_println!(
                "   Moonshot alignment: {:.1}% (hits: {})",
                self.moonshot_alignment * 100.0,
                hits
            );
        }
    }
}
