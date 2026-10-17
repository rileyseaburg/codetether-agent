//! Task signatures and relevance-weighted message scores.
use super::*;

/// Default recent-window size — last N entries always retained.
pub const DEFAULT_RECENT_WINDOW: usize = 8;

/// Per-overlap-file weight in the relevance score.
const FILE_OVERLAP_WEIGHT: f64 = 4.0;

/// Per-overlap-tool weight in the relevance score.
const TOOL_OVERLAP_WEIGHT: f64 = 2.5;

/// Per-error-class weight (boundary boost).
const ERROR_BOUNDARY_WEIGHT: f64 = 3.0;

/// Recency decay weight: each step away from the tail subtracts this much.
const RECENCY_DECAY_WEIGHT: f64 = 0.05;

/// Build a relevance signature from the most recent user turn.
pub fn task_signature(messages: &[Message]) -> RelevanceMeta {
    active_tail::active_user_tail_start(messages, 0)
        .and_then(|idx| messages.get(idx))
        .map(extract)
        .unwrap_or_default()
}

/// Score every message against `task`. Higher is more relevant.
pub fn score_messages(messages: &[Message], task: &RelevanceMeta) -> Vec<f64> {
    let n = messages.len();
    messages
        .iter()
        .enumerate()
        .map(|(i, msg)| {
            let meta = extract(msg);
            let mut score = 0.0;
            score += FILE_OVERLAP_WEIGHT * overlap_count(&meta.files, &task.files) as f64;
            score += TOOL_OVERLAP_WEIGHT * overlap_count(&meta.tools, &task.tools) as f64;
            score += ERROR_BOUNDARY_WEIGHT * meta.error_classes.len() as f64;
            // Recency: newer messages get a small bonus regardless of overlap.
            let distance_from_tail = (n - 1).saturating_sub(i) as f64;
            score -= RECENCY_DECAY_WEIGHT * distance_from_tail;
            score
        })
        .collect()
}

pub(super) fn overlap_count(left: &[String], right: &[String]) -> usize {
    if left.is_empty() || right.is_empty() {
        return 0;
    }
    let right_set: std::collections::HashSet<_> = right.iter().collect();
    left.iter().filter(|item| right_set.contains(item)).count()
}
