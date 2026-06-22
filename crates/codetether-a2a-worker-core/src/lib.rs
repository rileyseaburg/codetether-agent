//! Reusable A2A worker primitives shared by the main agent crate.

mod agent_routing;
mod capabilities;
mod cognition;
pub mod codetether_agent;
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
