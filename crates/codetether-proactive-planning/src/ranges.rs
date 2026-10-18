//! Plan bounded multi-scale ranges while retaining the active tail.
use super::windows::windows;
use crate::Plan;
use codetether_context_policy::summary_index::{Granularity, SummaryRange};

const ACTIVE_TAIL: usize = 8;

const TURN_SIZE: usize = 4;

const PHASE_SIZE: usize = 16;

const EPOCH_SIZE: usize = 64;

/// Plan aligned summary windows while leaving the active tail intact.
/// # Arguments
/// * `message_count` — Number of transcript messages currently available.
/// # Returns
/// Bounded turn, phase and session summary plans in their existing priority order.
/// # Examples
/// ```
/// assert!(codetether_proactive_planning::planned(11).is_empty());
/// assert!(codetether_proactive_planning::planned(10_000).len() <= 97);
/// ```
pub fn planned(message_count: usize) -> Vec<Plan> {
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
