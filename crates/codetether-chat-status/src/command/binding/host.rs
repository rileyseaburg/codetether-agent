//! Frontend status snapshot and report presentation.
/// Implementation detail of [crate::declare_frontend_status_command].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_status_command_host {
    ($message:path, $kind:path) => {
        mod host {
            use super::{App, Session};
            use $crate::command::{Snapshot, View};
            use $kind as MessageType;
            use $message as ChatMessage;
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub session: &'a Session,
            }
            impl View for Native<'_> {
                fn snapshot(&self) -> Snapshot<'_> {
                    Snapshot {
                        session_id: &self.session.id,
                        model: self.session.metadata.model.as_deref(),
                        auto_apply_edits: self.app.state.preferences.auto_apply_edits,
                        allow_network: self.app.state.preferences.allow_network,
                        use_worktree: self.app.state.preferences.use_worktree,
                        slash_autocomplete: self.app.state.preferences.slash_autocomplete,
                        context_used: self.app.state.context_health.used_tokens,
                        context_budget: self.app.state.context_health.token_budget,
                        last_completion_model: self.app.state.last_completion.model.as_deref(),
                        last_completion_prompt_tokens: self.app.state.last_completion.prompt_tokens,
                        last_completion_output_tokens: self.app.state.last_completion.output_tokens,
                    }
                }
                fn push_system(&mut self, text: String) {
                    self.app
                        .state
                        .messages
                        .push(ChatMessage::new(MessageType::System, text));
                }
                fn scroll_to_bottom(&mut self) {
                    self.app.state.chat_viewport.scroll_to_bottom();
                }
                fn clear_input(&mut self) {
                    self.app.state.composer.clear_input();
                }
                fn set_status(&mut self, status: String) {
                    self.app.state.status = status;
                }
            }
        }
    };
}
