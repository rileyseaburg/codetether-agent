//! Frontend health, token and message bindings for context telemetry.

/// Declare a context-event view using existing frontend and message types.
///
/// Health state, token counts and status remain in the frontend. Error text is
/// appended through its existing message constructor and message-type enum.
/// # Examples
/// ```text
/// codetether_context_health::declare_context_events!(App, SessionEvent, ChatMessage, MessageType);
/// ```
#[macro_export]
macro_rules! declare_context_events {
    ($app:ty, $event:ty, $message:path, $kind:path $(,)?) => {
        mod host {
            use $kind as MessageType;
            use $message as ChatMessage;
            pub(super) struct Native<'a>(pub &'a mut $app);
            impl $crate::events::View for Native<'_> {
                fn health(&mut self) -> &mut $crate::ContextHealthState {
                    &mut self.0.state.context_health
                }
                fn set_used(&mut self, tokens: usize) {
                    self.0.state.context_health.used_tokens = Some(tokens);
                }
                fn set_budget(&mut self, tokens: usize) {
                    self.0.state.context_health.token_budget = Some(tokens);
                }
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
                fn push_error(&mut self, text: String) {
                    self.0
                        .state
                        .messages
                        .push(ChatMessage::new(MessageType::Error, text));
                }
            }
        }
        pub(super) fn handle_event(app: &mut $app, evt: $event) {
            $crate::events::handle_event(&mut host::Native(app), evt);
        }
    };
}
