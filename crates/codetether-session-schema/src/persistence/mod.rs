//! Persistence entry points over the application's existing storage host.
//! [crate::impl_session_persistence!] adds loading, saving, history and paths.
mod history;
mod lifecycle;
mod paths;
mod tail;
mod workspace;

/// Add persistence methods to an application-local session schema.
///
/// The type must already implement the shared storage contracts and
/// `codetether_session_store::lifecycle::Host`. Listing, migration and
/// post-save notifications stay with that host implementation.
///
/// # Usage
/// After declaring the type and its host, invoke:
/// ```text
/// codetether_session_schema::impl_session_persistence!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_persistence {
    ($name:ident) => {
        $crate::__session_persistence_lifecycle!($name);
        $crate::__session_persistence_tail!($name);
        $crate::__session_persistence_history!($name);
        $crate::__session_persistence_paths!($name);
        $crate::__session_persistence_workspace!($name);
    };
}
