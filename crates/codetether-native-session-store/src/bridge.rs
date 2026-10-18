//! Application-local bridge to native persistence services.
/// Retain a local Session lifecycle implementation backed by native services.
/// # Usage
/// Implement Session in this crate's native contract, then invoke:
/// ```text
/// codetether_native_session_store::impl_session_lifecycle!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_lifecycle {
    ($session:ty) => {
        impl $crate::__private::Host for $session {
            type Session = $session;
            async fn load_path(
                path: &::std::path::Path,
                window: usize,
            ) -> $crate::__private::Result<$crate::__private::TailLoad<$session>> {
                <$crate::Host<$session> as $crate::__private::Host>::load_path(path, window).await
            }
            async fn list_session_ids(
                workspace: Option<&::std::path::Path>,
            ) -> $crate::__private::Result<Vec<String>> {
                <$crate::Host<$session> as $crate::__private::Host>::list_session_ids(workspace)
                    .await
            }
            fn schedule_recall(session: &$session) {
                <$crate::Host<$session> as $crate::__private::Host>::schedule_recall(session)
            }
            fn notify_saved(session: &$session) {
                <$crate::Host<$session> as $crate::__private::Host>::notify_saved(session)
            }
            fn schedule_archive(session: &$session) {
                <$crate::Host<$session> as $crate::__private::Host>::schedule_archive(session)
            }
            async fn notify_removed(id: &str) {
                <$crate::Host<$session> as $crate::__private::Host>::notify_removed(id).await
            }
            async fn remove_recall(id: &str) -> $crate::__private::Result<()> {
                <$crate::Host<$session> as $crate::__private::Host>::remove_recall(id).await
            }
        }
    };
}
