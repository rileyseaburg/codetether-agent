//! Worker execution defaults and result classification.
//!
//! [AutoApprove] parses the worker's automatic-tool policy. Model helpers
//! retain provider ordering and default model IDs; result helpers classify
//! session/swarm outcomes without invoking an executor.
//!
//! ```rust
//! use codetether_a2a_worker_core::execution_policy::{map_session_result, default_model_for_provider};
//! assert_eq!(default_model_for_provider("openai-codex", None), "gpt-5.5");
//! let outcome = map_session_result("partial".into(), "session".into(), true, 200);
//! assert_eq!(outcome.0, "failed");
//! ```
mod auto_approve;
mod model_defaults;
mod model_preferences;
mod registry;
pub use registry::Registry;
mod task_agent_swarm_result;
pub use auto_approve::{AutoApprove, parse_auto_approve};
pub use model_defaults::default_model_for_provider;
pub use model_preferences::choose_provider_for_tier;
pub use task_agent_swarm_result::{map_session_result, map_swarm_result};

/// Availability validation and effective provider/model selection.
pub mod model_selection;

mod tool_approval;
pub use tool_approval::{is_safe_tool, is_tool_allowed};

/// Worker agent selection and execution-result normalization.
pub mod agent;
