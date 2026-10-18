//! Relay session construction and owned prompt execution.
//!
//! [Session] supplies native session storage access. [Native] preserves relay
//! configuration order and returns the owned session even when prompting fails.
use codetether_bus_core::AgentBus;
use codetether_native_providers::ProviderRegistry;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_session_events::{SessionEvent, SessionResult};
use std::{future::Future, marker::PhantomData, sync::Arc};
use tokio::sync::mpsc;
mod create;
mod host;
mod prompt;
/// Application session operations used by a Ralph relay.
/// # Implementors
/// Implement with the application's normal creation, storage and prompt methods.
pub trait Session: Sized + Send + 'static {
    /// Create an empty session. Errors retain the application's creation error.
    fn new() -> impl Future<Output = anyhow::Result<Self>> + Send;
    /// Set the selected model, agent name, bus and system message in that order.
    fn configure(&mut self, model: String, name: String, bus: Arc<AgentBus>, message: Message);
    /// Prompt with the relay event stream. Errors retain the original prompt error.
    fn prompt_with_events(
        &mut self,
        prompt: &str,
        events: mpsc::Sender<SessionEvent<Self>>,
        registry: Arc<ProviderRegistry>,
    ) -> impl Future<Output = anyhow::Result<SessionResult>> + Send;
}
/// Relay host for an application session type.
/// # Examples
/// `Native<S>` implements the shared relay host when `S: Session`.
pub struct Native<S>(PhantomData<fn() -> S>);
