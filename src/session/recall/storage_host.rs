//! Shared recall storage host using the native session and projection reader.
codetether_recall_store::declare_session_host!(
    pub(crate) struct NativeHost;,
    crate::session::Session,
    crate::session::store::projection::read_session
);
