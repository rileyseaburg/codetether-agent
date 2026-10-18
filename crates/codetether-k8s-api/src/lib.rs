//! Kubernetes HTTP API operations on a caller-supplied manager.
//!
//! Native handlers supply their manager and audit log. ErrorMapper preserves
//! the caller's error-to-status policy; this crate owns validation and operation
//! order for status, scaling, restarting and subagent lifecycle requests.
mod actions;
mod delete;
mod pods;
mod requests;
mod restart;
mod scale;
mod spawn;
mod status;
pub use actions::k8s_actions;
pub use delete::k8s_delete_subagent;
pub use pods::k8s_list_pods;
pub use requests::{ScaleRequest, SpawnSubagentRequest};
pub use restart::k8s_restart;
pub use scale::k8s_scale;
pub use spawn::k8s_spawn_subagent;
pub use status::get_k8s_status;
/// Caller-owned mapping from manager errors to HTTP status and response text.
pub type ErrorMapper = fn(anyhow::Error) -> (axum::http::StatusCode, String);

mod workers;
pub use workers::list_connected_workers;

/// HTTP route groups over existing Kubernetes and audit services.
pub mod http;
