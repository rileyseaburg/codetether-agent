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
pub use codetether_bus_core as bus_durable_impl;
pub use codetether_bus_core as durability_class;
pub use codetether_bus_core::durable_log;
pub use codetether_bus_core::durable_log_file;
pub use codetether_bus_core::global;
pub use codetether_bus_core::payload;
pub mod publish_effects;
pub use codetether_bus_core as ralph_progress_publish;
pub use codetether_bus_core as ralph_publish;
pub use codetether_bus_core as voice_handle;
pub use codetether_bus_core::recorder;
pub use codetether_bus_core::registry;
pub use codetether_bus_core::relay;
pub use codetether_bus_core::speech;
pub use codetether_bus_core::tool_catalog;
pub use codetether_bus_s3 as s3_sink;

pub use global::{global, set_global};

pub use codetether_bus_core::{AgentBus, BusEnvelope, BusHandle, BusMessage};
