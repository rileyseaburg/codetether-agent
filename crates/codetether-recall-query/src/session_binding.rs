//! Bind recall queries to existing loading, projection and backfill operations.
pub use anyhow;
pub use async_trait::async_trait;
pub use codetether_recall_index::IndexedSession;
/// Bind the existing session loader and callbacks to a recall query host.
/// # Usage
/// ```text
/// codetether_recall_query::impl_session_host!(Host, Session, schedule_backfill, project_session);
/// ```
#[macro_export]
macro_rules! impl_session_host {
    ($host:ty, $session:ty, $schedule:path, $build:path) => {
        #[$crate::session_binding::async_trait]
        impl $crate::Host for $host {
            type Session = $session;
            fn schedule_backfill(&self, workspace: &::std::path::Path) {
                $schedule(workspace);
            }
            async fn load_tail(
                &self,
                id: &str,
                limit: usize,
            ) -> $crate::session_binding::anyhow::Result<$session> {
                Ok(<$session>::load_tail(id, limit).await?.session)
            }
            fn build(
                session: &$session,
            ) -> ::std::option::Option<$crate::session_binding::IndexedSession> {
                $build(session)
            }
        }
    };
}
