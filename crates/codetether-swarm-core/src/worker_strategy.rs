//! Worker task strategy-name aliases.
use crate::DecompositionStrategy;
/// Map an optional worker strategy name to a swarm decomposition strategy.
///
/// # Arguments
/// * `value` - Strategy selected from task metadata, with legacy names allowed.
/// # Returns
/// The corresponding strategy, defaulting to automatic decomposition.
/// # Examples
/// ```rust
/// use codetether_swarm_core::{DecompositionStrategy, worker_strategy::parse_worker_strategy};
/// assert!(matches!(parse_worker_strategy(Some("by_domain".into())), DecompositionStrategy::ByDomain));
/// ```
pub fn parse_worker_strategy(value: Option<String>) -> DecompositionStrategy {
    match value.as_deref().map(str::to_ascii_lowercase).as_deref() {
        Some("none") | Some("single") => DecompositionStrategy::None,
        Some("domain") | Some("by_domain") => DecompositionStrategy::ByDomain,
        Some("data") | Some("by_data") => DecompositionStrategy::ByData,
        Some("stage") | Some("by_stage") => DecompositionStrategy::ByStage,
        _ => DecompositionStrategy::Automatic,
    }
}
