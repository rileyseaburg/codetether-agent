//! Shared recall projection using the native session and canonicalizer.
codetether_recall_index::define_session_projection!(
    pub(super) fn session(crate::session::Session), super::paths::canonical
);
