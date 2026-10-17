//! A2A Server - serve as an A2A agent

use super::types::*;
use crate::session::SessionEvent;
use anyhow::Result;
use axum::Router;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::Instant;

use crate::bus::BusMessage;
use tokio::sync::mpsc;
use uuid::Uuid;

#[path = "server_admission_host.rs"]
mod admission_host;
#[path = "server_http_host.rs"]
mod http_host;
#[path = "server_message_send_turn.rs"]
mod server_message_send_turn;
#[path = "server_session_event.rs"]
mod server_session_event;
#[path = "server_turn.rs"]
mod server_turn;

mod bus;
mod card;
mod construction;
mod stream;
use bus::{emit_a2a_inbound, emit_a2a_outbound};
use stream::handle_message_stream;

/// A2A Server state
#[derive(Clone)]
pub struct A2AServer {
    tasks: Arc<DashMap<String, Task>>,
    agent_card: AgentCard,
    /// Optional bus for emitting session events for SSE streaming
    bus: Option<Arc<crate::bus::AgentBus>>,
}
