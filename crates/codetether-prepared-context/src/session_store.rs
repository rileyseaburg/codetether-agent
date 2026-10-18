//! Validated session sidecar paths and prepared-context operations.
pub use anyhow;
/// Declare the existing session-store helper functions using native path validation.
/// # Usage
/// ```text
/// codetether_prepared_context::declare_session_store!(Session);
/// ```
#[macro_export]
macro_rules! declare_session_store {
    ($session:ty) => {
        pub(super) use $crate::Prepared;
        pub(super) async fn read_id(session_id: &str) -> Option<Prepared> {
            let path = path(session_id).ok()?;
            $crate::read(&path).await
        }
        pub(super) async fn remove(session_id: &str) {
            if let Ok(path) = path(session_id) {
                $crate::remove(&path).await;
            }
        }
        pub(super) async fn write(
            session_id: &str,
            value: &Prepared,
        ) -> $crate::session_store::anyhow::Result<()> {
            let path = path(session_id)?;
            $crate::write(&path, value).await
        }

        fn path(session_id: &str) -> $crate::session_store::anyhow::Result<std::path::PathBuf> {
            <$session>::session_path(session_id)?;
            Ok(<$session>::sessions_dir()?
                .join("rlm")
                .join(format!("{session_id}.json")))
        }
    };
}
