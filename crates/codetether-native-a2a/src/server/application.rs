//! Preserve native session execution, provider caching and card version identity.
use crate::Session;
use codetether_native_providers::ProviderRegistry;
use codetether_session_events::{SessionEvent, SessionResult};
use std::{marker::PhantomData, sync::Arc};
use tokio::sync::mpsc;
pub struct Native<S>(PhantomData<fn() -> S>);
impl<S: Session> codetether_a2a_server::Application for Native<S> {
    type Session = S;
    type Registry = Arc<ProviderRegistry>;
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    async fn resolve(context_id: Option<&str>) -> anyhow::Result<S> {
        S::resolve(context_id).await
    }
    async fn configure(session: &mut S) {
        crate::session_config::configure(session).await;
    }
    async fn registry() -> anyhow::Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
    async fn execute(
        session: &mut S,
        prompt: &str,
        sender: mpsc::Sender<SessionEvent<S>>,
        registry: Arc<ProviderRegistry>,
    ) -> anyhow::Result<String> {
        session
            .prompt_with_events(prompt, sender, registry)
            .await
            .map(|result| result.text)
    }
    async fn prompt(session: &mut S, prompt: &str) -> anyhow::Result<SessionResult> {
        crate::prompt_runtime::run(session, prompt).await
    }
    async fn persist(session: &S) {
        crate::session_config::persist(session).await;
    }
}
