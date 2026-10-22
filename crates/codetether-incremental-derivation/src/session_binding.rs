//! Application-local transcript and prepared-summary view.
pub use async_trait::async_trait;
pub use codetether_context_policy::{pages::PageKind, summary_index::SummaryIndex};
pub use codetether_provider_core::Message;
/// Bind existing session fields and the prepared-index callback.
/// An optional third argument supplies the absolute loaded-window offset.
/// # Usage
/// ```text
/// codetether_incremental_derivation::impl_session_source!(Session, prepared_index);
/// ```
#[macro_export]
macro_rules! impl_session_source {
    ($session:ty, $prepared:path) => {
        $crate::impl_session_source!($session, $prepared, |_| 0);
    };
    ($session:ty, $prepared:path, $offset:expr) => {
        #[$crate::session_binding::async_trait]
        impl $crate::Source for $session {
            fn messages(&self) -> &[$crate::session_binding::Message] {
                &self.messages
            }
            fn pages(&self) -> &[$crate::session_binding::PageKind] {
                &self.pages
            }
            fn source_offset(&self) -> usize {
                ($offset)(self)
            }
            fn summary_index(&self) -> Option<&$crate::session_binding::SummaryIndex> {
                Some(&self.summary_index)
            }
            async fn prepared_index(&self) -> $crate::session_binding::SummaryIndex {
                $prepared(self).await
            }
        }
    };
}
