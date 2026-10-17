//! Worker strategy metadata key precedence.
use crate::task_config::metadata_str;
use codetether_swarm_core::DecompositionStrategy;
/// Select a swarm strategy using the worker's legacy metadata-key order.
///
/// # Arguments
/// * `metadata` - Worker task metadata.
/// # Returns
/// The selected strategy, defaulting to automatic decomposition.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::swarm_setup::parse_swarm_strategy;
/// let strategy = parse_swarm_strategy(&serde_json::Map::new());
/// assert!(matches!(strategy, codetether_swarm_core::DecompositionStrategy::Automatic));
/// ```
pub fn parse_swarm_strategy(
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
