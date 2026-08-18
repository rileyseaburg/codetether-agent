//! Memory snapshot construction during the Compress phase.

use chrono::Utc;
use uuid::Uuid;

use super::tick_snapshot_meta::metadata;
use super::{MemorySnapshot, ThoughtResult, ThoughtWorkItem, text_util};

/// Distill a Compress-phase thought into a memory snapshot.
pub(super) fn build_snapshot(
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
