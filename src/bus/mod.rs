//! Central in-process agent message bus
//!
//! Provides a broadcast-based pub/sub bus that all local agents (sub-agents,
//! workers, the A2A server) can plug into.  Remote agents talk to the bus
//! through the gRPC / JSON-RPC bridge; local agents get zero-copy clones
//! via `BusHandle`.
//!
//! # Topic routing
//!
//! Every envelope carries a `topic` string that follows a hierarchical scheme:
//!
//! | Pattern | Semantics |
//! |---------|-----------|
//! | `agent.{id}` | Messages *to* a specific agent |
//! | `agent.{id}.events` | Events *from* a specific agent |
//! | `task.{id}` | All updates for a task |
//! | `swarm.{id}` | Swarm-level coordination |
//! | `broadcast` | Global announcements |
//! | `tools.{name}` | Tool-specific channels |

pub mod registry;

use crate::a2a::types::{Artifact, Part, TaskState};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

// ─── Envelope & Messages ─────────────────────────────────────────────────

/// Metadata wrapper for every message that travels through the bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusEnvelope {
    /// Unique id for this envelope
    pub id: String,
    /// Hierarchical topic for routing (e.g. `agent.{id}`, `task.{id}`)
    pub topic: String,
    /// The agent that originated this message
    pub sender_id: String,
    /// Optional correlation id (links request → response)
    pub correlation_id: Option<String>,
    /// When the envelope was created
    pub timestamp: DateTime<Utc>,
    /// The payload
    pub message: BusMessage,
}

/// The set of messages the bus can carry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BusMessage {
    /// An agent has come online and is ready to receive work
    AgentReady {
        agent_id: String,
        capabilities: Vec<String>,
    },
    /// An agent is shutting down
    AgentShutdown { agent_id: String },
    /// Free-form message from one agent to another (mirrors A2A `Message`)
    AgentMessage {
        from: String,
        to: String,
        parts: Vec<Part>,
    },
    /// Task status changed
    TaskUpdate {
        task_id: String,
        state: TaskState,
        message: Option<String>,
    },
    /// A new artifact was produced for a task
    ArtifactUpdate { task_id: String, artifact: Artifact },
    /// A sub-agent published a shared result (replaces raw `ResultStore` publish)
    SharedResult {
        key: String,
        value: serde_json::Value,
        tags: Vec<String>,
    },
    /// Tool execution request (for shared tool dispatch)
    ToolRequest {
        request_id: String,
        agent_id: String,
        tool_name: String,
        arguments: serde_json::Value,
    },
    /// Tool execution response
    ToolResponse {
        request_id: String,
        agent_id: String,
        tool_name: String,
        result: String,
        success: bool,
    },
    /// Heartbeat (keep-alive / health signal)
    Heartbeat { agent_id: String, status: String },
}

// ─── Constants ───────────────────────────────────────────────────────────

/// Default channel capacity (per bus instance)
const DEFAULT_BUS_CAPACITY: usize = 4096;

// ─── AgentBus ────────────────────────────────────────────────────────────

/// The central in-process message bus.
///
/// Internally this is a `tokio::sync::broadcast` channel so every subscriber
/// receives every message.  Filtering by topic is done on the consumer side
/// through `BusHandle::subscribe_topic` / `BusHandle::recv_filtered`.
pub struct AgentBus {
    tx: broadcast::Sender<BusEnvelope>,
    /// Registry of connected agents
    pub registry: Arc<registry::AgentRegistry>,
}

impl AgentBus {
    /// Create a new bus with default capacity.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BUS_CAPACITY)
    }

    /// Create a new bus with a specific channel capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            tx,
            registry: Arc::new(registry::AgentRegistry::new()),
        }
    }

    /// Wrap `self` in an `Arc` for sharing across tasks.
    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }

    /// Create a `BusHandle` scoped to a specific agent.
    pub fn handle(self: &Arc<Self>, agent_id: impl Into<String>) -> BusHandle {
        BusHandle {
            agent_id: agent_id.into(),
            bus: Arc::clone(self),
            rx: self.tx.subscribe(),
        }
    }

    /// Publish an envelope directly (low-level).
    pub fn publish(&self, envelope: BusEnvelope) -> usize {
        match &envelope.message {
            BusMessage::AgentReady {
                agent_id,
                capabilities,
            } => {
                self.registry.register_ready(agent_id, capabilities);
            }
            BusMessage::AgentShutdown { agent_id } => {
                self.registry.deregister(agent_id);
            }
            _ => {}
        }

        // Returns the number of receivers that got the message.
        // If there are no receivers the message is silently dropped.
        self.tx.send(envelope).unwrap_or(0)
    }

    /// Number of active receivers.
    pub fn receiver_count(&self) -> usize {
        self.tx.receiver_count()
    }
}

impl Default for AgentBus {
    fn default() -> Self {
        Self::new()
    }
}

// ─── BusHandle ───────────────────────────────────────────────────────────

/// A scoped handle that a single agent uses to send and receive messages.
///
/// The handle knows the agent's id and uses it as `sender_id` on outgoing
/// envelopes.  It also provides filtered receive helpers.
pub struct BusHandle {
    agent_id: String,
    bus: Arc<AgentBus>,
    rx: broadcast::Receiver<BusEnvelope>,
}

impl BusHandle {
    /// The agent id this handle belongs to.
    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    /// Send a message on the given topic.
    pub fn send(&self, topic: impl Into<String>, message: BusMessage) -> usize {
        self.send_with_correlation(topic, message, None)
    }

    /// Send a message with a correlation id.
    pub fn send_with_correlation(
        &self,
        topic: impl Into<String>,
        message: BusMessage,
        correlation_id: Option<String>,
    ) -> usize {
        let envelope = BusEnvelope {
            id: Uuid::new_v4().to_string(),
            topic: topic.into(),
            sender_id: self.agent_id.clone(),
            correlation_id,
            timestamp: Utc::now(),
            message,
        };
        self.bus.publish(envelope)
    }

    /// Announce this agent as ready.
    pub fn announce_ready(&self, capabilities: Vec<String>) -> usize {
        self.send(
            "broadcast",
            BusMessage::AgentReady {
                agent_id: self.agent_id.clone(),
                capabilities,
            },
        )
    }

    /// Announce this agent is shutting down.
    pub fn announce_shutdown(&self) -> usize {
        self.send(
            "broadcast",
            BusMessage::AgentShutdown {
                agent_id: self.agent_id.clone(),
            },
        )
    }

    /// Announce a task status update.
    pub fn send_task_update(
        &self,
        task_id: &str,
        state: TaskState,
        message: Option<String>,
    ) -> usize {
        self.send(
            format!("task.{task_id}"),
            BusMessage::TaskUpdate {
                task_id: task_id.to_string(),
                state,
                message,
            },
        )
    }

    /// Announce an artifact update.
    pub fn send_artifact_update(&self, task_id: &str, artifact: Artifact) -> usize {
        self.send(
            format!("task.{task_id}"),
            BusMessage::ArtifactUpdate {
                task_id: task_id.to_string(),
                artifact,
            },
        )
    }

    /// Send a direct message to another agent.
    pub fn send_to_agent(&self, to: &str, parts: Vec<Part>) -> usize {
        self.send(
            format!("agent.{to}"),
            BusMessage::AgentMessage {
                from: self.agent_id.clone(),
                to: to.to_string(),
                parts,
            },
        )
    }

    /// Publish a shared result (visible to the entire swarm).
    pub fn publish_shared_result(
        &self,
        key: impl Into<String>,
        value: serde_json::Value,
        tags: Vec<String>,
    ) -> usize {
        let key = key.into();
        self.send(
            format!("results.{}", &key),
            BusMessage::SharedResult { key, value, tags },
        )
    }

    /// Receive the next envelope (blocks until available).
    pub async fn recv(&mut self) -> Option<BusEnvelope> {
        loop {
            match self.rx.recv().await {
                Ok(env) => return Some(env),
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    tracing::warn!(
                        agent_id = %self.agent_id,
                        skipped = n,
                        "Bus handle lagged, skipping messages"
                    );
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }

    /// Receive the next envelope whose topic starts with the given prefix.
    pub async fn recv_topic(&mut self, prefix: &str) -> Option<BusEnvelope> {
        loop {
            match self.recv().await {
                Some(env) if env.topic.starts_with(prefix) => return Some(env),
                Some(_) => continue, // wrong topic, skip
                None => return None,
            }
        }
    }

    /// Receive the next envelope addressed to this agent.
    pub async fn recv_mine(&mut self) -> Option<BusEnvelope> {
        let prefix = format!("agent.{}", self.agent_id);
        self.recv_topic(&prefix).await
    }

    /// Try to receive without blocking. Returns `None` if no message is
    /// queued.
    pub fn try_recv(&mut self) -> Option<BusEnvelope> {
        loop {
            match self.rx.try_recv() {
                Ok(env) => return Some(env),
                Err(broadcast::error::TryRecvError::Lagged(n)) => {
                    tracing::warn!(
                        agent_id = %self.agent_id,
                        skipped = n,
                        "Bus handle lagged (try_recv), skipping"
                    );
                    continue;
                }
                Err(broadcast::error::TryRecvError::Empty)
                | Err(broadcast::error::TryRecvError::Closed) => return None,
            }
        }
    }

    /// Access the agent registry.
    pub fn registry(&self) -> &Arc<registry::AgentRegistry> {
        &self.bus.registry
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bus_send_recv() {
        let bus = AgentBus::new().into_arc();
        let mut handle_a = bus.handle("agent-a");
        let mut handle_b = bus.handle("agent-b");

        handle_a.send_to_agent(
            "agent-b",
            vec![Part::Text {
                text: "hello".into(),
            }],
        );

        // Both handles receive the broadcast
        let env = handle_b.recv().await.unwrap();
        assert_eq!(env.topic, "agent.agent-b");
        match &env.message {
            BusMessage::AgentMessage { from, to, .. } => {
                assert_eq!(from, "agent-a");
                assert_eq!(to, "agent-b");
            }
            other => panic!("unexpected message: {other:?}"),
        }

        // handle_a also receives it (broadcast semantics)
        let env_a = handle_a.try_recv().unwrap();
        assert_eq!(env_a.topic, "agent.agent-b");
    }

    #[tokio::test]
    async fn test_bus_task_update() {
        let bus = AgentBus::new().into_arc();
        let handle = bus.handle("worker-1");

        let h2 = bus.handle("observer");
        // need mutable for recv
        let mut h2 = h2;

        handle.send_task_update("task-42", TaskState::Working, Some("processing".into()));

        let env = h2.recv().await.unwrap();
        assert_eq!(env.topic, "task.task-42");
        match &env.message {
            BusMessage::TaskUpdate { task_id, state, .. } => {
                assert_eq!(task_id, "task-42");
                assert_eq!(*state, TaskState::Working);
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_bus_no_receivers() {
        let bus = AgentBus::new().into_arc();
        // No handles created — send should succeed with 0 receivers
        let env = BusEnvelope {
            id: "test".into(),
            topic: "broadcast".into(),
            sender_id: "nobody".into(),
            correlation_id: None,
            timestamp: Utc::now(),
            message: BusMessage::Heartbeat {
                agent_id: "nobody".into(),
                status: "ok".into(),
            },
        };
        let count = bus.publish(env);
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_recv_topic_filter() {
        let bus = AgentBus::new().into_arc();
        let handle = bus.handle("agent-x");
        let mut listener = bus.handle("listener");

        // Send to two different topics
        handle.send(
            "task.1",
            BusMessage::TaskUpdate {
                task_id: "1".into(),
                state: TaskState::Working,
                message: None,
            },
        );
        handle.send(
            "task.2",
            BusMessage::TaskUpdate {
                task_id: "2".into(),
                state: TaskState::Completed,
                message: None,
            },
        );

        // recv_topic("task.2") should skip the first and return the second
        let env = listener.recv_topic("task.2").await.unwrap();
        match &env.message {
            BusMessage::TaskUpdate { task_id, .. } => assert_eq!(task_id, "2"),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_ready_shutdown_syncs_registry() {
        let bus = AgentBus::new().into_arc();
        let handle = bus.handle("planner-1");

        handle.announce_ready(vec!["plan".to_string(), "review".to_string()]);
        assert!(bus.registry.get("planner-1").is_some());

        handle.announce_shutdown();
        assert!(bus.registry.get("planner-1").is_none());
    }
}
