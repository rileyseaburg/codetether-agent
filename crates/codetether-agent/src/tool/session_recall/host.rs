//! Select native storage and bus types for shared indexed recall and synthesis.
/// Recall host retaining native storage access and the session event type.
pub type ApplicationHost = codetether_session_recall::application::ApplicationHost<
    crate::session::SessionBus,
    codetether_recall_query::tool::IndexedSearch<
        crate::session::index::recall::storage_host::NativeHost,
    >,
>;
