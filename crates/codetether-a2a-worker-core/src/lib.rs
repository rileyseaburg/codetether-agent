//! Reusable A2A worker primitives shared by the main agent crate.

mod agent_routing;
mod capabilities;
pub mod codetether_agent;
mod cognition;
mod cognition_env;
mod heartbeat;
mod heartbeat_ops;
mod identity;
mod runtime_env;

pub use agent_routing::{is_forage_agent, is_swarm_agent, model_ref_to_provider_model};
pub use capabilities::{DEFAULT_A2A_SERVER_URL, advertised_interfaces, worker_capabilities};
pub use cognition::{CognitionHeartbeatConfig, CognitionLatestSnapshot, CognitionStatusSnapshot};
pub use heartbeat::{HeartbeatState, WorkerStatus};
pub use identity::{generate_worker_id, normalize_max_concurrent_tasks, resolve_worker_id};
pub use runtime_env::export_worker_runtime_env;

#[cfg(test)]
mod consolidation;

/// CloudEvent contracts and task-ingress normalization.
pub mod cloudevents;

/// Task-envelope targeting policy for worker intake.
pub mod task_scope;

/// Task-envelope parsing and worker configuration policies.
pub mod task_config;

/// Worker approval, model defaults and result classification.
pub mod execution_policy;

/// Task progress, deadline accounting and checkpoint diagnostics.
pub mod task_timeline;

/// Model, agent and host metadata for worker registration.
pub mod registration;

/// Task-claim wire fields and provenance mapping.
pub mod claim;

/// Task targeting, capacity reservations and execution deadlines.
pub mod dispatch;
