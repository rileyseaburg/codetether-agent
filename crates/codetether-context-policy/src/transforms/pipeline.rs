//! Ordered default transforms with a host reasoning-repair hook.
use super::{ExperimentalStats, dedup, lingua, pairing, thinking_prune};
use codetether_provider_core::Message;
/// Apply default transforms in their fixed order around one repair callback.
///
/// # Arguments
/// * `messages` - Conversation history transformed in place.
/// * `repair_reasoning` - Host hook called after thinking pruning.
///
/// # Returns
/// Aggregate transform counts and bytes saved.
///
/// # Examples
/// ```
/// use codetether_context_policy::transforms::{pipeline::apply_all, ExperimentalStats};
/// let stats = apply_all(&mut Vec::new(), |_| ExperimentalStats::default());
/// assert_eq!(stats.total_bytes_saved, 0);
/// ```
pub fn apply_all(
    messages: &mut Vec<Message>,
    repair_reasoning: impl FnOnce(&mut Vec<Message>) -> ExperimentalStats,
) -> ExperimentalStats {
    let mut stats = ExperimentalStats::default();
    stats.merge(thinking_prune::prune_thinking(messages));
    // Repair: ensure pruned reasoning_content is non-null for
    // DeepSeek V4 (runs via bundled tetherscript hook).
    stats.merge(repair_reasoning(messages));
    stats.merge(dedup::dedup_tool_outputs(messages));
    stats.merge(lingua::prune_low_entropy(messages));
    // Correctness pass: repair any orphaned tool_call/tool_result
    // pairs broken by the strategies above. Must run LAST.
    stats.merge(pairing::repair_orphans(messages));

    stats
}
