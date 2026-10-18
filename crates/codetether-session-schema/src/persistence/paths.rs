//! paths entry points for application-local session types.
/// Internal persistence methods for the shared session declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_persistence_paths {
    ($name:ident) => {
        impl $name {
            /// Resolve the configured sessions directory.
            pub(crate) fn sessions_dir() -> $crate::__private::anyhow::Result<::std::path::PathBuf>
            {
                $crate::__private::location::sessions_dir()
            }

            /// Resolve a durable session by ID across mux worktree ownership changes.
            pub(crate) fn session_path(
                id: &str,
            ) -> $crate::__private::anyhow::Result<::std::path::PathBuf> {
                $crate::__private::store::lifecycle::session_path(id)
            }
        }
    };
}
