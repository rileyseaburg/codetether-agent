//! Frontend state and message bindings for context-status commands.

/// Declare context-status commands using the existing frontend state.
///
/// Health, token estimates and system notices retain their frontend storage and
/// message types. Parsing and report rendering remain in the shared command.
/// # Examples
/// ```text
/// codetether_context_health::declare_context_command!(App, ChatMessage, MessageType);
/// ```
#[macro_export]
macro_rules! declare_context_command {
    ($app:ty, $message:path, $kind:path $(,)?) => {
        mod context_host {
            use $kind as MessageType;
            use $message as ChatMessage;
            impl $crate::command::View for $app {
                fn context(&self) -> (&$crate::ContextHealthState, Option<usize>, Option<usize>) {
                    (
                        &self.state.context_health,
                        self.state.context_health.used_tokens,
                        self.state.context_health.token_budget,
                    )
                }
                fn push_system_message(&mut self, text: String) {
                    self.state
                        .messages
                        .push(ChatMessage::new(MessageType::System, text));
                }
                fn scroll_to_bottom(&mut self) {
                    self.state.chat_viewport.scroll_to_bottom();
                }
                fn set_status(&mut self, text: String) {
                    self.state.status = text;
                }
            }
        }
        /// Dispatch a context-status command using shared report formatting.
        fn handle_context_command(app: &mut $app, rest: &str) {
            $crate::command::execute(app, rest);
        }
    };
}
