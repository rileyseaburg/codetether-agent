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
pub mod bus_durable_impl;
pub mod durability_class;
pub use codetether_bus_core::durable_log;
pub mod durable_log_file;
pub mod global;
pub mod payload;
pub mod publish_effects;
pub mod ralph_progress_publish;
pub mod ralph_publish;
pub mod recorder;
pub mod registry;
pub mod relay;
pub mod s3_sink;
pub mod speech;
pub mod tool_catalog;
pub mod voice_handle;

pub use global::{global, set_global};

pub use codetether_bus_core::{AgentBus, BusEnvelope, BusHandle, BusMessage};
