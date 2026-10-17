//! Shared forage normalization and worker strategy metadata lookup.
use super::metadata_strings::metadata_str;
use crate::swarm::DecompositionStrategy;
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
