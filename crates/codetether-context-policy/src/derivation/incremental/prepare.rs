//! Score, reserve protected entries and order optional history before index lookup.
use super::*;
/// Prepare selection without reading live session state or an index.
///
/// # Arguments
/// * `clone` - Owned transcript projection to select from.
/// * `origin_len` - Original canonical transcript length.
/// * `budget_tokens` - Maximum request budget used by this derivation.
/// * `estimate_request` - Estimate including the host system prompt and tools.
/// * `estimate_messages` - Estimate message content without fixed overhead.
/// * `seed_required` - Mark protected entries and debit their costs.
///
/// # Returns
/// An owned plan to retain while the host obtains its summary index.
///
/// # Examples
/// See the construction example on [PreparedIncremental].
pub fn prepare(
    clone: Vec<Message>,
    origin_len: usize,
    budget_tokens: usize,
    estimate_request: impl Fn(&[Message]) -> usize,
    estimate_messages: impl Fn(&[Message]) -> usize,
    seed_required: impl FnOnce(&mut [bool], &[usize], usize, &mut usize),
) -> PreparedIncremental {
    let task = task_signature(&clone);
    let scores = score_messages(&clone, &task);

    let recent_start = clone.len().saturating_sub(DEFAULT_RECENT_WINDOW);

    let header_cost = budget_tokens.saturating_sub(estimate_request(&[])).max(1);
    // Token budget for the message slice itself, after system + tools.
    let mut budget_for_messages = header_cost;

    let per_msg = clone
        .iter()
        .map(|m| estimate_messages(std::slice::from_ref(m)))
        .collect::<Vec<usize>>();

    // Always-include set: recent-window entries + pinned constraints.
    let mut keep = vec![false; clone.len()];
    seed_required(&mut keep, &per_msg, recent_start, &mut budget_for_messages);

    // Greedy pack the older entries by descending score.
    let mut order: Vec<usize> = (0..recent_start).collect();
    order.sort_by(|a, b| {
        scores[*b]
            .partial_cmp(&scores[*a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    PreparedIncremental {
        clone,
        origin_len,
        budget_tokens,
        keep,
        per_msg,
        order,
        budget_for_messages,
    }
}
