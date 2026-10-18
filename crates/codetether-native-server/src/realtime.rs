//! Realtime HTTP execution using native provider and session services.
use codetether_native_providers::ProviderRegistry;
use codetether_session_events::{SessionEvent, SessionResult};
use codetether_session_metadata::SessionMetadata;
use std::{future::Future, marker::PhantomData, sync::Arc};
use tokio::sync::mpsc;

/// Strict session loading and metadata access for realtime requests.
/// # Implementors
/// Applications retain their existing load operation, separate from A2A
/// context resolution, while reusing native A2A prompt and save behavior.
pub trait RealtimeSession: codetether_native_a2a::Session {
    /// Load an existing session without creating a replacement.
    /// # Errors
    /// Returns the application's normal session loading errors.
    fn load_realtime(id: &str) -> impl Future<Output = anyhow::Result<Self>> + Send;
    /// Borrow metadata for explicit per-request model selection.
    fn realtime_metadata(&mut self) -> &mut SessionMetadata;
}

/// Shared realtime host with the native provider cache and application session.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_native_server::RealtimeSession>() {
/// let _routes = codetether_session_realtime::http::router::<codetether_native_server::RealtimeHost<S>, ()>();
/// # }
/// ```
pub struct RealtimeHost<S>(PhantomData<fn() -> S>);
impl<S: RealtimeSession> codetether_session_realtime::Host for RealtimeHost<S> {
    type Session = S;
    type Registry = ProviderRegistry;
    async fn load(id: &str) -> anyhow::Result<S> {
        S::load_realtime(id).await
    }
    fn metadata(session: &mut S) -> &mut SessionMetadata {
        session.realtime_metadata()
    }
    async fn load_registry() -> anyhow::Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
    async fn prompt(
        session: &mut S,
        message: &str,
        events: mpsc::Sender<SessionEvent<S>>,
        registry: Arc<ProviderRegistry>,
    ) -> anyhow::Result<SessionResult> {
        session.prompt_with_events(message, events, registry).await
    }
    async fn save(session: &S) -> anyhow::Result<()> {
        session.save().await
    }
}
