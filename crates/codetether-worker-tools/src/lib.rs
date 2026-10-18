//! Workspace-aware tool assembly for A2A workers.
//!
//! Safe and mutating groups preserve worker registration order and approval
//! policy. [Host] supplies the executable registry and application-bound tools.
//! [create_filtered_registry] always appends the invalid-tool fallback.
//!
//! # Examples
//! ```no_run
//! use codetether_worker_tools::{Host, create_filtered_registry};
//! use codetether_a2a_worker_core::execution_policy::AutoApprove;
//! use codetether_provider_core::Provider;
//! use std::{path::Path, sync::Arc};
//! fn build<H: Host>(provider: Arc<dyn Provider>) -> H::Registry {
//!     create_filtered_registry::<H>(
//!         provider, "example/model".into(), AutoApprove::Safe, Path::new("."), None,
//!     )
//! }
//! ```
mod build;
mod host;
mod mutating;
mod safe;
pub use build::create_filtered_registry;
pub use host::{Host, Registry, Workflow, Workspace};
pub use mutating::{
    register_edit_tools, register_go_tool, register_model_tools, register_mutating_tools,
    register_task_tools,
};
pub use safe::register_safe_tools;
