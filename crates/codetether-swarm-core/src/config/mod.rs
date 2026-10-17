//! Shared swarm execution settings and decomposition policy.
//!
//! [SwarmConfig] supplies resource, retry and execution defaults. Parse a CLI
//! mode through [ExecutionMode::from_cli_value] and select a [DecompositionStrategy].
use serde::{Deserialize, Serialize};
mod defaults;
use defaults::{
    default_base_delay_ms, default_k8s_pod_budget, default_max_delay_ms, default_max_retries,
};
mod limits;
pub use limits::{MAX_SUBAGENTS, MAX_TOOL_CALLS};
mod settings;
pub use settings::SwarmConfig;
mod execution_mode;
mod settings_default;
pub use execution_mode::ExecutionMode;
mod decomposition;
pub use decomposition::DecompositionStrategy;
