//! Frontend queue operations for queued-prompt delivery.
/// Implementation detail of [crate::declare_frontend_queued_prompt].
#[doc(hidden)]
#[macro_export]
macro_rules! __queued_prompt_queue {
    ($pending:path, $held:path, $take:path) => {
        mod queue {
            use super::Native;

            impl $crate::queued::Queue for Native<'_> {
                fn processing(&self) -> bool {
                    self.app.state.processing
                }
                fn has_pending(&self) -> bool {
                    $pending()
                }
                fn held(&self) -> bool {
                    $held(self.slot.view().id())
                }
                fn take(&mut self) -> Option<String> {
                    $take()
                }
            }
        }
    };
}
