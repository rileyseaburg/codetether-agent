//! Worker swarm execution settings.
use codetether_swarm_core::{DecompositionStrategy, SwarmConfig};
/// Configuration and decomposition strategy selected for a worker swarm task.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::swarm_setup::SwarmSetup;
/// use codetether_swarm_core::DecompositionStrategy;
/// let setup = SwarmSetup { config: Default::default(), strategy: DecompositionStrategy::Automatic };
/// assert!(matches!(setup.strategy, DecompositionStrategy::Automatic));
/// ```

pub struct SwarmSetup {
    /// Resource and execution configuration.
    pub config: SwarmConfig,
    /// Selected task-decomposition strategy.
    pub strategy: DecompositionStrategy,
}
