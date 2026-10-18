//! Frontend relay presentation and state access.
/// Implementation detail of [crate::declare_frontend_autochat].
#[doc(hidden)]
#[macro_export]
macro_rules! __autochat_view {
    ($state:path, $event:ty, $message:path, $kind:path) => {
        use super::state::AutochatState;
        use $crate::interactive::host::View;
        use $kind as MessageType;
        use $message as ChatMessage;
        use $state as AppState;
        impl View<$event> for AppState {
            fn autochat(&mut self) -> &mut AutochatState {
                &mut self.autochat
            }
            fn last_completion_model(&self) -> Option<String> {
                self.last_completion.model.clone()
            }
            fn set_status(&mut self, status: String) {
                self.status = status;
            }
            fn push_system(&mut self, text: String) {
                self.messages
                    .push(ChatMessage::new(MessageType::System, text));
            }
            fn scroll_to_bottom(&mut self) {
                AppState::scroll_to_bottom(self);
            }
        }
    };
}
