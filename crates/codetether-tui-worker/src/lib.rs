//! TUI worker bridge orchestration over shared A2A and event transports.
//!
//! [spawn] resolves configuration and starts registration, heartbeat, inbound
//! task streaming and bus forwarding. [Registration] supplies host metadata.
mod commands;
mod config;
mod context;
mod context_init;
mod control;
mod heartbeat;
mod identity;
mod registration;
mod runtime;
mod spawn;
mod stream;
mod stream_read;
mod stream_request;
mod types;
pub use codetether_worker_events::IncomingTask;
pub use registration::{Registration, RegistrationRequest};
pub use spawn::spawn;
pub use types::{Bridge, WorkerBridgeCmd};
mod registered;
pub use registered::RegisteredBridge;

mod worker_identity;
mod worker_queue;
mod worker_recent;
mod worker_state;
pub use worker_state::WorkerState;

mod shutdown;
/// Ordered deregistration and bounded bridge-task shutdown.
pub use shutdown::shutdown_bridge;

/// Incoming task ingestion and idle-time presentation.
pub mod tasks;

/// UI processing and observed-agent synchronization with the bridge.
pub mod synchronization;

/// Application-local state method declarations.
pub mod state_binding;
