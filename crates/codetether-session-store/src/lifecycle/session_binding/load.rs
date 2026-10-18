//! Existing session load callbacks for the shared persistence host.
/// Internal persistence host methods.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_store_load {
    ($session:ty, $load:path, $workspace:path, $all:path) => {
        async fn load_path(
            path: &::std::path::Path,
            window: usize,
        ) -> $crate::lifecycle::session_binding::anyhow::Result<$crate::TailLoad<$session>> {
            $load(path, window).await
        }
        async fn list_session_ids(
            workspace: ::std::option::Option<&::std::path::Path>,
        ) -> $crate::lifecycle::session_binding::anyhow::Result<
            ::std::vec::Vec<::std::string::String>,
        > {
            let sessions = match workspace {
                Some(workspace) => $workspace(workspace).await?,
                None => $all().await?,
            };
            Ok(sessions.into_iter().map(|session| session.id).collect())
        }
    };
}
