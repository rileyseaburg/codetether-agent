//! Selector healing search.

use super::score::{element_similarity, ElementProps};

/// Find the best matching candidate when the original selector fails.
/// Returns the index of the best match above the threshold.
pub fn heal_selector(
    original_props: &ElementProps,
    candidates: &[ElementProps],
    threshold: f64,
) -> Option<usize> {
    let mut best = None;
    let mut best_score = threshold;
    for (i, c) in candidates.iter().enumerate() {
        let score = element_similarity(original_props, c);
        if score >= best_score {
            best = Some(i);
            best_score = score;
        }
    }
    best
}
