//! Reusable worker startup identity, workspace parsing and processing state.
//! [parse_codebases] retains configured workspace order. [resolve_name] binds
//! the runtime identity. [processing_set] creates the shared active-task set.
mod codebases;
mod name;
mod processing;
pub use codebases::parse_codebases;
pub use name::resolve_name;
pub use processing::{ProcessingSet, processing_set};

mod runtime;
mod runtime_clone;
mod context;
mod build;
mod runtime_config;
pub use runtime::WorkerTaskRuntime;
pub use context::WorkerContext;
pub use build::build_task_runtime;
pub use runtime_config::RuntimeConfig;

/// Startup registration, workspace synchronization and task intake.
pub mod bootstrap;
