//! Bounded multi-scale ranges anticipated for future context retrieval.

use crate::session::index::{Granularity, SummaryRange};

const ACTIVE_TAIL: usize = 8;
const TURN_SIZE: usize = 4;
const PHASE_SIZE: usize = 16;
const EPOCH_SIZE: usize = 64;
const WINDOWS_PER_SCALE: usize = 32;

#[derive(Clone, Copy)]
pub(super) struct Plan {
    pub range: SummaryRange,
    pub granularity: Granularity,
    pub target_tokens: usize,
}

pub(super) fn planned(message_count: usize) -> Vec<Plan> {
    let end = message_count.saturating_sub(ACTIVE_TAIL);
    if end < TURN_SIZE {
        return Vec::new();
    }
    let mut plans = windows(end, TURN_SIZE, Granularity::Turn, 192);
    plans.extend(windows(end, PHASE_SIZE, Granularity::Phase, 512));
    plans.extend(windows(end, EPOCH_SIZE, Granularity::Session, 768));
    let checkpoint = end / EPOCH_SIZE * EPOCH_SIZE;
    if checkpoint >= EPOCH_SIZE {
        let range = SummaryRange::new(0, checkpoint).expect("non-empty prefix checkpoint");
        if !plans.iter().any(|plan| plan.range == range) {
            plans.push(Plan {
                range,
                granularity: Granularity::Session,
                target_tokens: 1_024,
            });
        }
    }
    plans
}

fn windows(end: usize, size: usize, granularity: Granularity, target_tokens: usize) -> Vec<Plan> {
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
