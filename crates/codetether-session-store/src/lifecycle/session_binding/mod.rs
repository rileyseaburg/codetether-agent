//! Bind existing loaders, listings and post-persistence callbacks to a session.
mod load;
mod notifications;
pub use anyhow;
/// Implement the persistence host using the application's existing callbacks.
///
/// Callback order is preserved by the shared lifecycle implementation.
/// # Usage
/// ```text
/// codetether_session_store::impl_session_lifecycle_host!(Session;
///     load, list_workspace, list_all, schedule_recall, notify_saved,
///     schedule_archive, notify_removed, remove_recall);
/// ```
#[macro_export]
macro_rules! impl_session_lifecycle_host {
    ($session:ty; $load:path, $workspace:path, $all:path, $recall:path, $saved:path,
     $archive:path, $removed:path, $remove_recall:path) => {
        impl $crate::lifecycle::Host for $session {
            type Session = $session;
            $crate::__session_store_load!($session, $load, $workspace, $all);
            $crate::__session_store_notifications!(
                $session,
                $recall,
                $saved,
                $archive,
                $removed,
                $remove_recall
            );
        }
    };
}
