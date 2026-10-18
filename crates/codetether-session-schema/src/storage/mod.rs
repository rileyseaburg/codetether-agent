//! Snapshot, restoration and import contracts over the shared session fields.
//! [crate::impl_session_store!] binds storage and header projection to the local type.
mod header;
mod legacy;
mod restore;
mod snapshot;

/// Add shared persistence contracts to an application-local session type.
///
/// Invoke after [crate::declare_session!] and [crate::impl_session_lifecycle!].
/// # Examples
/// ```no_run
/// codetether_session_schema::declare_session!(pub struct Session;);
/// codetether_session_schema::impl_session_lifecycle!(Session);
/// codetether_session_schema::impl_session_store!(Session);
/// let session = Session::default();
/// let header = codetether_session_schema::header(&session);
/// assert!(header.id.is_empty());
/// ```
#[macro_export]
macro_rules! impl_session_store {
    ($name:ident) => {
        $crate::__session_storage_header!($name);
        $crate::__session_storage_snapshot!($name);
        $crate::__session_storage_restore!($name);
        $crate::__session_storage_legacy!($name);
    };
}
