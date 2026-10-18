//! Existing session notifications callbacks for the shared persistence host.
/// Internal persistence host methods.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_store_notifications {
    ($session:ty, $recall:path, $saved:path, $archive:path, $removed:path, $remove_recall:path) => {
        fn schedule_recall(session: &$session) {
            $recall(session);
        }
        fn notify_saved(session: &$session) {
            $saved(session);
        }
        fn schedule_archive(session: &$session) {
            $archive(session);
        }
        async fn notify_removed(id: &str) {
            $removed(id).await;
        }
        async fn remove_recall(id: &str) -> $crate::lifecycle::session_binding::anyhow::Result<()> {
            $remove_recall(id).await
        }
    };
}
