//! Application-local proactive worker storage and snapshot operations.
pub use anyhow;
pub use async_trait::async_trait;
pub use codetether_prepared_context::Prepared;
pub use codetether_provider_core::Message;
pub use codetether_session_store::{Delivery, Ticket, window::WINDOW};
pub use serde_json::Value;
/// Declare a worker host using existing session and persistence operations.
///
/// The projection callback returns an acceptance flag; this unit-returning
/// worker contract retains errors and discards the successful flag.
/// # Usage
/// ```text
/// codetether_proactive_worker::declare_session_host!(pub struct Host;
///     Session, next, commit, read_prepared, write_prepared, remove_prepared);
/// ```
#[macro_export]
macro_rules! declare_session_host {
    ($visibility:vis struct $host:ident; $(,)? $session:ty,
     $next:path, $commit:path, $read:path, $write:path, $remove:path) => {
        $visibility struct $host;
        #[$crate::session_binding::async_trait]
impl $crate::Host for $host {
    type Session = $session;
    fn message_count(session: &$session) -> usize {
        session.messages.len()
    }
    fn into_messages(session: $session) -> ::std::vec::Vec<$crate::session_binding::Message> {
        session.messages.into_vec()
    }
    async fn next(
        &self,
        id: &str,
        consumer: &str,
        limit: usize,
    ) -> $crate::session_binding::anyhow::Result<::std::option::Option<$crate::session_binding::Delivery<$session>>> {
        $next(id, consumer, limit).await
    }
    async fn commit(&self, ticket: $crate::session_binding::Ticket, view: $crate::session_binding::Value) -> $crate::session_binding::anyhow::Result<()> {
        $commit(ticket, view).await.map(|_| ())
    }
    async fn load_tail(&self, id: &str) -> $crate::session_binding::anyhow::Result<::std::vec::Vec<$crate::session_binding::Message>> {
        let loaded = <$session>::load_tail(id, $crate::session_binding::WINDOW).await?;
        Ok(loaded.session.messages.into_vec())
    }
    async fn read_prepared(&self, id: &str) -> ::std::option::Option<$crate::session_binding::Prepared> {
        $read(id).await
    }
    async fn write_prepared(&self, id: &str, value: &$crate::session_binding::Prepared) -> $crate::session_binding::anyhow::Result<()> {
        $write(id, value).await
    }
    async fn remove_prepared(&self, id: &str) {
        $remove(id).await;
    }
}

    };
}
