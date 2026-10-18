//! Frontend ui effects for continuation and session resume.
/// Implementation detail of [crate::declare_frontend_continuation].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_continuation_ui {
    ($notification:path, $message:path, $publish:path) => {
        mod ui {
            use super::Native;
            use $message as ChatMessage;
            use $notification as WatchdogNotification;
            impl $crate::resume::Ui for Native<'_> {
                fn clear_input(&mut self) {
                    self.app.state.composer.clear_input();
                }
                fn set_restarts(&mut self, count: u32) {
                    self.app.state.watchdog.restart_count = count;
                }
                fn set_inflight(&mut self, prompt: String) {
                    self.app.state.watchdog.inflight_prompt = Some(prompt);
                }
                fn arm_retry(&mut self, message: String, count: u32) {
                    self.app.state.watchdog.notification =
                        Some(WatchdogNotification::new(message, count));
                }
                fn set_processing(&mut self, processing: bool) {
                    self.app.state.processing = processing;
                }
                fn clear_streaming(&mut self) {
                    self.app
                        .state
                        .message_cache
                        .clear_streaming_text(&mut self.app.state.streaming);
                }
                fn clear_timing(&mut self) {
                    self.app.state.clear_request_timing();
                }
                fn status(&mut self, message: String) {
                    self.app.state.status = message;
                }
                fn messages_mut(&mut self) -> &mut Vec<ChatMessage> {
                    &mut self.app.state.messages
                }
                fn publish_user(&mut self, prompt: &str) {
                    $publish(self.app, prompt, &[]);
                }
                fn scroll(&mut self) {
                    self.app.state.scroll_to_bottom();
                }
            }
        }
    };
}
