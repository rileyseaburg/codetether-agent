//! Direct swarm tool, invocation contracts and execution policy.
//!
//! Parse tool JSON with [input::parse_tasks] and [execution_config::ExecutionConfig],
//! resolve the requested model with [model_selection::resolve], and serialize
//! worker results with [aggregate_response::build] and [aggregate::result].
//! [SwarmExecuteTool] executes through the shared [runtime], using
//! [runtime::Host] adapters for providers, worker registries and UI observation.
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

/// Repository-aware direct-worker system prompt construction.
pub mod prompt;
/// Registry lookup and provider-error reporting for direct workers.
pub mod provider_setup;

mod tool;
mod tool_impl;
/// Direct-swarm tool using the application runtime selected by its type parameter.
pub use tool::SwarmExecuteTool;
