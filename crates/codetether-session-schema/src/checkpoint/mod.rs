//! Checkpoint sidecar operations over existing session metadata and persistence.
//! [crate::impl_session_checkpoints!] binds the sidecar store and its public API.
mod api;
mod host;
/// Add checkpoint sidecar methods and storage callbacks to a local session.
///
/// Requires the session persistence methods from [crate::impl_session_persistence!].
/// # Usage
/// ```text
/// codetether_session_schema::impl_session_checkpoints!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_checkpoints {
    ($name:ident) => {
        $crate::__session_checkpoint_host!($name);
        $crate::__session_checkpoint_api!($name);
    };
}
