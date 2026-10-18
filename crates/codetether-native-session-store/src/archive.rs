//! Coalesced history archival using native durable consumer snapshots.
use crate::Session;
use codetether_history_sink::archive::ArchiveSource;
use codetether_provider_core::Message;
use codetether_session_store::{Delivery, Snapshot};
use std::marker::PhantomData;
/// Native archive source retaining the application's session decoder.
/// # Examples
/// ```
/// let _: Option<codetether_native_session_store::archive::Archive<()>> = None;
/// ```
pub struct Archive<S>(PhantomData<fn() -> S>);
#[async_trait::async_trait]
impl<S: Session> ArchiveSource for Archive<S> {
    type Session = S;
    async fn next(id: &str, name: &str, limit: usize) -> anyhow::Result<Option<Delivery<S>>> {
        codetether_session_store::consumer::next_for::<S>(id, name, limit).await
    }
    fn messages(session: &S) -> &[Message] {
        Snapshot::messages(session)
    }
}
/// Schedule archival when the session has a configured history sink.
/// # Arguments
/// * session - Persisted session whose durable ID selects consumer state.
/// # Returns
/// Immediately after scheduling the shared archive worker, or if disabled.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_native_session_store::Session>(session: &S) {
/// codetether_native_session_store::archive::schedule(session);
/// # }
/// ```
pub fn schedule<S: Session>(session: &S) {
    let Some(config) = session.history_sink() else {
        return;
    };
    let id = Snapshot::id(session).to_owned();
    codetether_history_sink::archive::schedule::<Archive<S>>(id, config);
}
