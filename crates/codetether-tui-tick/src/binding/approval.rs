//! Frontend approval queue and overlay state access.
/// Declare approval reconciliation over the existing queue and frontend state.
///
/// # Examples
/// ```text
/// codetether_tui_tick::declare_frontend_approval!(App, remove_stale, remaining);
/// ```
#[macro_export]
macro_rules! declare_frontend_approval {
    ($app:path, $remove:path, $remaining:path $(,)?) => {
        use $app as App;
        use $crate::approval::{Host, State};
        struct Native<'a>(&'a mut App);
        impl Host for Native<'_> {
            fn remove_stale(&mut self) -> usize {
                $remove()
            }
            fn remaining(&self) -> usize {
                $remaining()
            }
            fn state(&mut self) -> State<'_> {
                State {
                    waiting: &mut self.0.state.approval_waiting,
                    preview_scroll: &mut self.0.state.approval_preview_scroll,
                    status: &mut self.0.state.status,
                }
            }
        }
        pub(super) fn reconcile(app: &mut App) -> bool {
            $crate::approval::reconcile(&mut Native(app))
        }
    };
}
