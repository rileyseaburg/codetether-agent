//! In-process broadcast delivery shared by local agents.
//!
//! Create an [AgentBus], wrap it with [AgentBus::into_arc], and obtain a
//! scoped [BusHandle] to publish envelopes and consume live messages.
use crate::speech::SpeechAct;
use crate::{BusEnvelope, BusMessage, durable_log, recorder, registry, tool_catalog};
use codetether_a2a_core::{Artifact, Part, TaskState};
use std::sync::Arc;
use tokio::sync::broadcast;
const DEFAULT_BUS_CAPACITY: usize = 4096;

// ─── AgentBus ────────────────────────────────────────────────────────────

/// The central in-process message bus.
///
/// Internally this is a `tokio::sync::broadcast` channel so every subscriber
/// receives every message.  Filtering by topic is done on the consumer side
/// through [BusHandle::recv_topic] / [BusHandle::recv_mine].
///
/// # Examples
/// ```
/// use codetether_bus_core::AgentBus;
/// let bus = AgentBus::new().into_arc();
/// let handle = bus.handle("worker");
/// assert_eq!(handle.agent_id(), "worker");
/// ```
pub struct AgentBus {
    tx: broadcast::Sender<BusEnvelope>,
    /// Registry of connected agents
    pub registry: Arc<registry::AgentRegistry>,
    /// Optional durable log: when set, [`AgentBus::publish`] persists
    /// coordination messages for replay. `None` keeps legacy lossy behavior.
    durable: Option<Arc<dyn durable_log::DurableLog>>,
    /// In-memory ring buffer of recent envelopes for on-demand inspection.
    pub recorder: Arc<recorder::BusRecorder>,
}

// ─── BusHandle ───────────────────────────────────────────────────────────

/// A scoped handle that a single agent uses to send and receive messages.
///
/// The handle knows the agent's id and uses it as `sender_id` on outgoing
/// envelopes.  It also provides filtered receive helpers.
///
/// # Examples
/// ```
/// use codetether_bus_core::AgentBus;
/// let bus = AgentBus::new().into_arc();
/// let mut handle = bus.handle("worker");
/// assert!(handle.try_recv().is_none());
/// ```
pub struct BusHandle {
    agent_id: String,
    bus: Arc<AgentBus>,
    rx: broadcast::Receiver<BusEnvelope>,
}
include!("modules.rs");
