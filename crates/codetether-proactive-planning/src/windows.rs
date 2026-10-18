//! Generate the bounded aligned windows for one summary scale.
use crate::Plan;
use codetether_context_policy::summary_index::{Granularity, SummaryRange};

const WINDOWS_PER_SCALE: usize = 32;

pub(super) fn windows(
    end: usize,
    size: usize,
    granularity: Granularity,
    target_tokens: usize,
) -> Vec<Plan> {
    let aligned_end = end / size * size;
    let start = aligned_end.saturating_sub(size * WINDOWS_PER_SCALE);
    (start..aligned_end)
        .step_by(size)
        .filter_map(|offset| {
            SummaryRange::new(offset, offset + size).map(|range| Plan {
                range,
                granularity,
                target_tokens,
            })
        })
        .collect()
}
