//! Shared forage normalization and worker strategy metadata lookup.
use super::metadata_strings::metadata_str;
use crate::swarm::DecompositionStrategy;
pub(super) use codetether_forage_core::settings::{
    normalize_forage_execution_engine, normalize_forage_swarm_strategy,
};
pub(super) fn parse_swarm_strategy(
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> DecompositionStrategy {
    codetether_swarm_core::worker_strategy::parse_worker_strategy(metadata_str(
        metadata,
        &[
            "decomposition_strategy",
            "swarm_strategy",
            "strategy",
            "swarm_decomposition",
        ],
    ))
}
