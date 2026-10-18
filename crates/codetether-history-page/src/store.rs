//! History page reads backed by the indexed session store.
//!
//! [`Source`] retains the application's session decoding and lifecycle host,
//! while returning chronological message vectors for pagination.
use crate::HistorySource;
use codetether_provider_core::Message;
use codetether_session_store::{legacy::Legacy, lifecycle::Host};
use std::marker::PhantomData;

/// Bind history pagination to a session's persistence implementation.
/// # Examples
/// ```
/// # fn example<S: codetether_session_store::legacy::Legacy + codetether_session_store::lifecycle::Host<Session = S> + Send + Sync>() {
/// fn accepts_source<H: codetether_history_page::HistorySource>() {}
/// accepts_source::<codetether_history_page::store::Source<S>>();
/// # }
/// ```
pub struct Source<S>(PhantomData<fn() -> S>);

#[async_trait::async_trait]
impl<S: Legacy + Host<Session = S> + Send + Sync> HistorySource for Source<S> {
    async fn load_tail(id: &str, window: usize) -> Result<Vec<Message>, String> {
        codetether_session_store::lifecycle::load_tail::<S>(id, window)
            .await
            .map(|mut loaded| {
                std::mem::take(loaded.session.messages_mut())
                    .into_iter()
                    .collect()
            })
            .map_err(|error| error.to_string())
    }
    async fn read_messages(id: &str, start: usize, limit: usize) -> Result<Vec<Message>, String> {
        codetether_session_store::read_messages::<S>(id, start, limit)
            .await
            .map_err(|error| error.to_string())
    }
}
