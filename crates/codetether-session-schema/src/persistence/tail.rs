//! tail entry points for application-local session types.
/// Internal persistence methods for the shared session declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_persistence_tail {
    ($name:ident) => {
        impl $name {
            /// Load a session by UUID, keeping only the last `window` messages and
            /// tool uses. Use `window = 0` to drop all messages (cheap directory
            /// scan).
            ///
            /// # Errors
            /// Returns an error if the file cannot be opened or the JSON is malformed.
            pub async fn load_tail(
                id: &str,
                window: usize,
            ) -> $crate::__private::anyhow::Result<$crate::__private::store::TailLoad<Self>> {
                $crate::__private::store::lifecycle::load_tail::<Self>(id, window).await
            }

            /// Load a session from an explicit path with a bounded message window.
            pub async fn load_tail_from_path(
                path: ::std::path::PathBuf,
                window: usize,
            ) -> $crate::__private::anyhow::Result<$crate::__private::store::TailLoad<Self>> {
                <Self as $crate::__private::store::lifecycle::Host>::load_path(&path, window).await
            }
        }
    };
}
