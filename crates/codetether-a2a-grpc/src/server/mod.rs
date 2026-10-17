//! Shared gRPC task state and RPC dispatch.
//!
//! [GrpcTaskStore] stores tasks and emits status updates. [A2aServiceImpl]
//! adapts that state to generated tonic services; the host mounts the service.
use crate::proto::a2a_service_server::{A2aService, A2aServiceServer};
use crate::{bridge, proto};
use codetether_a2a_core::{self as local, TaskState};
use codetether_bus_core::AgentBus;
use dashmap::DashMap;
use std::{pin::Pin, sync::Arc};
use tokio::sync::broadcast;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};

type StreamResult<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

/// Shared state backing both JSON-RPC and gRPC transports.
///
/// Stores task records, push-notification configuration, agent metadata, and
/// task-update fan-out channels for the gRPC service layer. The store is safe to
/// share across concurrent request handlers: task and push-configuration maps
/// use lock-free concurrent access through [`DashMap`], while task updates are
/// distributed with a Tokio broadcast channel.
///
/// # Responsibilities
///
/// * Keep the authoritative in-process [`local::Task`] records by task ID.
/// * Track per-task [`local::TaskPushNotificationConfig`] entries used by
///   push-notification subscribers.
/// * Expose the local [`local::AgentCard`] advertised by this server.
/// * Optionally publish task lifecycle events through [`AgentBus`].
/// * Broadcast task status changes to active streaming subscribers.
///
/// # Invariants
///
/// * Keys in `tasks` and `push_configs` are task IDs.
/// * Values sent through `update_tx` use the same task ID namespace as `tasks`.
/// * `card` describes the agent instance served by this store.
pub struct GrpcTaskStore {
    tasks: DashMap<String, local::Task>,
    push_configs: DashMap<String, Vec<local::TaskPushNotificationConfig>>,
    card: local::AgentCard,
    bus: Option<Arc<AgentBus>>,
    /// Broadcast for task updates (task_id, status)
    ///
    /// Sends each observed task status transition to subscribers as a
    /// `(task_id, status)` tuple. Receivers may lag or miss messages according
    /// to Tokio broadcast-channel semantics, so consumers should treat this as
    /// a live update stream rather than durable task history.
    update_tx: broadcast::Sender<(String, local::TaskStatus)>,
}

/// Tonic service implementation.
pub struct A2aServiceImpl {
    store: Arc<GrpcTaskStore>,
}
mod service;
mod store;
