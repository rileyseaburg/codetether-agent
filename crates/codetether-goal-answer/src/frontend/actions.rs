//! Frontend actions binding for human-answer workflows.
/// Implementation detail of [crate::declare_frontend_answer_hosts].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_answer_actions {
    ($restart:path) => {
        mod actions {
            use super::Native;
            impl $crate::InputActions for Native<'_> {
                type Journal = super::super::review_host::Native;
                fn session_id(&self) -> String {
                    self.slot.view().id().to_string()
                }
                fn cancel(&mut self) {
                    self.runtime.request_cancel_current();
                }
                fn restart(&mut self, id: &str) {
                    $restart(self.app, id, self.runtime);
                }
            }
        }
    };
}
