//! Resolve and resume persisted workspace sessions through the existing store.
/// Internal workspace persistence methods for the shared session declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_persistence_workspace {
    ($name:ident) => {
        impl $name {
            /// Resolve the workspace recorded for a durable session.
            /// # Errors
            /// Returns identity, path, storage, JSON or missing-workspace errors.
            pub async fn recorded_workspace(
                id: &str,
            ) -> $crate::__private::anyhow::Result<::std::path::PathBuf> {
                let path = Self::session_path(id)?;
                $crate::__private::store::metadata::workspace::recorded_workspace(&path, id).await
            }

            /// Resume the latest workspace session without materializing its full history.
            ///
            /// # Arguments
            /// * `workspace` - Optional workspace filter; `None` uses the configured store.
            /// # Returns
            /// A bounded working tail with the original durable identity.
            /// # Errors
            /// Returns lookup, import, or database errors; absence does not erase history.
            /// # Examples
            /// ```rust,no_run
            /// # async fn example() -> anyhow::Result<()> {
            /// let session = codetether_agent::session::Session::resume_last(None).await?;
            /// assert!(session.messages.len() <= codetether_agent::session::store::WINDOW);
            /// # Ok(()) }
            /// ```
            pub async fn resume_last(
                workspace: ::std::option::Option<&::std::path::Path>,
            ) -> $crate::__private::anyhow::Result<Self> {
                Ok(Self::last_for_directory_tail(
                    workspace,
                    $crate::__private::store::window::WINDOW,
                )
                .await?
                .session)
            }
        }
    };
}
