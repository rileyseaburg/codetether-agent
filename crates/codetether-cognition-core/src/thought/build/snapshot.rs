//! Memory snapshot construction during the Compress phase.

use chrono::Utc;
use uuid::Uuid;

use super::metadata::metadata;
use crate::{MemorySnapshot, ThoughtResult, ThoughtWorkItem, text as text_util};

/// Distill a Compress-phase thought into a memory snapshot.
///
/// # Arguments
/// * `work` - Persona, phase and lineage associated with the completed work.
/// * `thought` - Completed thought and its provenance.
/// * `hot_event_count` - Number of recent events represented by the snapshot.
///
/// # Returns
/// The constructed record with the existing field and provenance rules.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::thought::build::build_snapshot;
/// let work = ThoughtWorkItem { persona_id: "p1".into(), persona_name: "Reviewer".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, thought_count: 1, phase: ThoughtPhase::Reflect };
/// let thought = ThoughtResult { source: "example", model: None, finish_reason: None, thinking: "Review the change".into(), prompt_tokens: None, completion_tokens: None, total_tokens: None, latency_ms: 0, error: None };
/// let snapshot = build_snapshot(&work, &thought, 3);
/// assert_eq!(snapshot.hot_event_count, 3);
/// ```
pub fn build_snapshot(
    work: &ThoughtWorkItem,
    thought: &ThoughtResult,
    hot_event_count: usize,
) -> MemorySnapshot {
    MemorySnapshot {
        id: Uuid::new_v4().to_string(),
        generated_at: Utc::now(),
        swarm_id: work.swarm_id.clone(),
        persona_scope: vec![work.persona_id.clone()],
        summary: text_util::trim_for_storage(&thought.thinking, 1_500),
        hot_event_count,
        warm_fact_count: text_util::estimate_fact_count(&thought.thinking),
        cold_snapshot_count: 1,
        metadata: metadata(work, thought),
    }
}
