use super::*;

/// Decomposition strategy for breaking down tasks
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
///
/// # Examples
/// ```
/// use codetether_swarm_core::DecompositionStrategy;
/// let strategy = DecompositionStrategy::default();
/// match strategy {
///     DecompositionStrategy::Automatic => {},
///     DecompositionStrategy::ByDomain | DecompositionStrategy::ByData |
///     DecompositionStrategy::ByStage | DecompositionStrategy::None => panic!("unexpected default"),
/// }
/// ```
pub enum DecompositionStrategy {
    /// Let the AI decide how to decompose
    #[default]
    Automatic,

    /// Decompose by domain/specialty
    ByDomain,

    /// Decompose by data partition
    ByData,

    /// Decompose by workflow stage
    ByStage,

    /// Single agent (no decomposition)
    None,
}
