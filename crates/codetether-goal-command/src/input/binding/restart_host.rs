//! Frontend cancellation and stale-retry cleanup.
/// Implementation detail of [crate::declare_frontend_goal_input].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_goal_restart_host {
    () => {
        mod restart_host {
            use super::*;
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub runtime: &'a TuiSessionHandle,
            }
            impl $crate::input::Restart for Native<'_> {
                fn cancel(&mut self) {
                    self.runtime.request_cancel_current();
                }
                fn clear_watchdog(&mut self) {
                    self.app.state.watchdog_notification = None;
                }
                fn clear_inflight(&mut self) {
                    self.app.state.main_inflight_prompt = None;
                }
            }
        }
    };
}
