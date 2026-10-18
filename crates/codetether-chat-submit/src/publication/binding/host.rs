//! Frontend processing and display effects for missing-provider feedback.
/// Implementation detail of [crate::declare_frontend_publication].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_publication_host {
    ($message:path, $stopped:path) => {
        mod publication_host {
            use super::{App, TuiWorkerBridge};
            use $message as ChatMessage;
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub bridge: &'a Option<TuiWorkerBridge>,
            }
            impl $crate::publication::FailureHost for Native<'_> {
                async fn processing_stopped(&mut self) {
                    $stopped(self.app, self.bridge).await;
                }
                fn clear_timing(&mut self) {
                    self.app.state.request_timing.clear_request_timing();
                }
                fn push(&mut self, message: ChatMessage) {
                    self.app.state.messages.push(message);
                }
                fn status(&mut self, message: String) {
                    self.app.state.status = message;
                }
                fn scroll(&mut self) {
                    self.app.state.chat_viewport.scroll_to_bottom();
                }
            }
        }
    };
}
