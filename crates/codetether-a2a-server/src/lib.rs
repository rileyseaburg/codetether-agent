//! A2A task admission, authenticated routing and streamed session execution.
//!
//! [A2AServer] owns task state and bus publication. [Application] supplies the
//! session and provider operations used to execute accepted prompts. Construct
//! a server with an agent card, optionally attach a bus, then consume its router.
use anyhow::Result;
use axum::Router;
use codetether_a2a_core::*;
use codetether_session_events::SessionEvent;
use dashmap::DashMap;
use std::{marker::PhantomData, sync::Arc};
use tokio::sync::mpsc;
mod application;
pub use application::Application;
mod admission_host;
mod bus;
mod card;
mod construction;
mod http_host;
mod server_message_send_turn;
mod server_session_event;
mod server_turn;
mod stream;
use bus::{emit_a2a_inbound, emit_a2a_outbound};
use stream::handle_message_stream;

/// A2A task state and routing bound to an application's session services.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_a2a_server::Application>() {
/// let card = codetether_a2a_server::A2AServer::<H>::default_card("http://localhost:8000");
/// let _server = codetether_a2a_server::A2AServer::<H>::new(card);
/// # }
/// ```
pub struct A2AServer<H: Application> {
    tasks: Arc<DashMap<String, Task>>,
    agent_card: AgentCard,
    bus: Option<Arc<codetether_bus_core::AgentBus>>,
    application: PhantomData<fn() -> H>,
}
impl<H: Application> Clone for A2AServer<H> {
    fn clone(&self) -> Self {
        Self {
            tasks: self.tasks.clone(),
            agent_card: self.agent_card.clone(),
            bus: self.bus.clone(),
            application: PhantomData,
        }
    }
}
