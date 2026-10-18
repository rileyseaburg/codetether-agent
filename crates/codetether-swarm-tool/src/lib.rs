//! Direct swarm invocation contracts and policy.
//!
//! Parse tool JSON with [input::parse_tasks] and [execution_config::ExecutionConfig],
//! resolve the requested model with [model_selection::resolve], and serialize
//! worker results with [aggregate_response::build] and [aggregate::result].
//! Runtime execution, provider loading, and UI observation belong to adapters.
/// Map an aggregate response into the tool success/failure envelope.
pub mod aggregate;
/// Serialize worker results and requested aggregation status.
pub mod aggregate_response;
/// Invocation concurrency, model, step and timeout settings.
pub mod execution_config;
/// Parse string and object task inputs.
pub mod input;
mod input_error;
#[cfg(test)]
mod input_tests;
/// Swarm-specific defaults, retained separately from ordinary session defaults.
pub mod model_defaults;
/// Extract explicit or inherited model references.
pub mod model_request;
mod model_resolution;
/// Resolve provider and model identities for diagnostics and execution.
pub mod model_selection;
#[cfg(test)]
mod model_selection_tests;
/// Provider-facing JSON schema for the swarm tool.
pub mod schema;
/// Task identity, instruction and worktree intent.
pub mod task_input;
/// Worker completion and failure records.
pub mod task_result;
#[cfg(test)]
mod task_result_tests;
/// Convert invocation tasks to worktree allocation specifications.
pub mod worktree_specs;

mod task_parse;

/// Direct worker cancellation on dropped join handles.
pub mod abort_on_drop;
/// Task intent classification and workspace selection.
pub mod support;
mod support_task;
/// Ordered allocation and integration of direct swarm worktrees.
pub mod worktrees;

mod workspace;

/// Concurrent direct-swarm execution with native runtime bindings.
pub mod runtime;
