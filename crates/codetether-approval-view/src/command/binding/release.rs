//! Frontend view binding for bulk approval release.
/// Declare the bulk-release wrapper for an existing approval view adapter.
/// # Examples
/// See the frontend approval release module for concrete state and host selection.
#[macro_export]
macro_rules! declare_frontend_approval_release {
    ($app:path, $host:path $(,)?) => {
        use $app as App;
        /// Release queued approvals through shared command policy.
        /// # Errors
        /// Returns the first shared approval release error.
        pub(crate) fn all(app: &mut App) -> Result<usize, String> {
            $crate::command::release_all(&mut $host(app))
        }
    };
}
