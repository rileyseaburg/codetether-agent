//! CLI decomposition names and the default automatic strategy.
use codetether_swarm_core::DecompositionStrategy;
pub(super) fn resolve(strategy: &str) -> DecompositionStrategy {
    match strategy {
        "auto" => DecompositionStrategy::Automatic,
        "domain" => DecompositionStrategy::ByDomain,
        "data" => DecompositionStrategy::ByData,
        "stage" => DecompositionStrategy::ByStage,
        "none" => DecompositionStrategy::None,
        _ => DecompositionStrategy::Automatic,
    }
}
