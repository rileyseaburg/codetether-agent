//! Frontend goal-command composer and transcript effects.
/// Implementation detail of [crate::declare_frontend_goal_input].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_goal_input_view {
    ($message:path, $kind:path) => {
        mod view {
            use super::Native;
            use $kind as MessageType;
            use $message as ChatMessage;
            impl $crate::input::View for Native<'_> {
                fn input(&self) -> &str {
                    &self.app.state.input
                }
                fn status(&mut self, text: String) {
                    self.app.state.status = text;
                }
                fn history(&mut self, text: String) {
                    self.app.state.push_history(text);
                }
                fn clear(&mut self) {
                    self.app.state.clear_input();
                }
                fn redraw(&mut self) {
                    self.app.state.needs_redraw = true;
                }
                fn system(&mut self, text: String) {
                    self.app
                        .state
                        .messages
                        .push(ChatMessage::new(MessageType::System, text));
                }
                fn scroll(&mut self) {
                    self.app.state.scroll_to_bottom();
                }
            }
        }
    };
}
