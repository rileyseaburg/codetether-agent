//! Existing listing, cursor and commit operations for recall workers.
pub use anyhow;
pub use async_trait::async_trait;
pub use codetether_session_store::{Delivery, Ticket};
pub use serde_json::Value;
/// Bind worker persistence to the application's existing callbacks.
/// # Usage
/// ```text
/// codetether_recall_worker::impl_session_host!(Host, Session, list, next, commit);
/// ```
#[macro_export]
macro_rules! impl_session_host {
    ($host:ty, $session:ty, $list:path, $next:path, $commit:path) => {
        #[$crate::session_binding::async_trait]
        impl $crate::Host for $host {
            async fn list_sessions(
                &self,
                workspace: &::std::path::Path,
            ) -> $crate::session_binding::anyhow::Result<::std::vec::Vec<::std::string::String>>
            {
                Ok($list(workspace)
                    .await?
                    .into_iter()
                    .map(|summary| summary.id)
                    .collect())
            }
            async fn next(
                &self,
                id: &str,
                consumer: &str,
                limit: usize,
            ) -> $crate::session_binding::anyhow::Result<
                ::std::option::Option<$crate::session_binding::Delivery<$session>>,
            > {
                $next(id, consumer, limit).await
            }
            async fn commit(
                &self,
                ticket: $crate::session_binding::Ticket,
                view: $crate::session_binding::Value,
            ) -> $crate::session_binding::anyhow::Result<bool> {
                $commit(ticket, view).await
            }
        }
    };
}
