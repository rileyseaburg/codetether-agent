//! Host console rendering for a shared forage opportunity.
use super::ForageOpportunity;
use crate::forage_println;

/// Print a single ranked opportunity entry (quiet-gated).
pub(super) fn print_cycle_entry(item: &ForageOpportunity, rank: usize, show_moonshot: bool) {
    forage_println!(
        "\n{}. [{}] {}",
        rank,
        item.okr_status_label(),
        item.okr_title
    );
    forage_println!(
        "   KR: {} ({:.1}% remaining, score {:.3})",
        item.key_result_title,
        item.remaining * 100.0,
        item.score
    );
    forage_println!(
        "   Progress: {:.1}% complete",
        item.progress.clamp(0.0, 1.0) * 100.0
    );
    if show_moonshot {
        let hits = if item.moonshot_hits.is_empty() {
            "none".to_string()
        } else {
            item.moonshot_hits.join(" | ")
        };
        forage_println!(
            "   Moonshot alignment: {:.1}% (hits: {})",
            item.moonshot_alignment * 100.0,
            hits
        );
    }
}
