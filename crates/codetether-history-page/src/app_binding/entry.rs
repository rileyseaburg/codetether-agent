//! History-page drain and pending-anchor entry points on local app types.
/// Internal history entry-point declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __app_history_entry {
    ($app:ty, $state:ty) => {
        /// Apply one completed page, if available.
        pub(crate) fn drain(app: &mut $app) -> bool {
            $crate::interactive::drain(&mut app.state)
        }
        impl $state {
            /// Restore the viewport anchor after older messages were prepended.
            pub(crate) fn apply_pending_history_anchor(&mut self, total_lines: usize) {
                $crate::interactive::apply_pending_history_anchor(self, total_lines);
            }
        }
    };
}
