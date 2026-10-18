//! Optional refinement, context and spawn counters.
use crate::Summary;
pub(super) fn append(summary: &mut String, input: &Summary<'_>) {
    let rlm_handoff_count = input.rlm_handoff_count;
    let rlm_context_count = input.rlm_context_count;
    let dynamic_spawn_count = input.dynamic_spawn_count;
    if rlm_handoff_count > 0 {
        summary.push_str(&format!("\nRLM-normalized handoffs: {rlm_handoff_count}"));
    }
    if rlm_context_count > 0 {
        summary.push_str(&format!("\nRLM context deltas: {rlm_context_count}"));
    }
    if input.shared_context_items > 0 {
        summary.push_str(&format!(
            "\nShared context items: {}",
            input.shared_context_items
        ));
    }
    if dynamic_spawn_count > 0 {
        summary.push_str(&format!("\nDynamic relay spawns: {dynamic_spawn_count}"));
    }
}
