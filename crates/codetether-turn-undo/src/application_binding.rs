//! Frontend session and view bindings for undo commands.
/// Declare undo command adapters using existing frontend and session state.
///
/// The notice callback retains application message presentation. Session history,
/// metadata and persistence operations remain on the selected session type.
/// # Examples
/// ```text
/// codetether_turn_undo::declare_undo_command!(App, Session, push_notice);
/// ```
#[macro_export]
macro_rules! declare_undo_command {
    ($app:ty, $session:ty, $notice:path $(,)?) => {
        mod undo_session {
            impl $crate::command::Session for $session {
                fn history(
                    &mut self,
                ) -> (
                    &mut Vec<$crate::__private::Message>,
                    &mut Vec<$crate::__private::PageKind>,
                ) {
                    (&mut self.messages, &mut self.pages)
                }
                fn set_updated_at(
                    &mut self,
                    timestamp: $crate::__private::DateTime<$crate::__private::Utc>,
                ) {
                    self.updated_at = timestamp;
                }
                async fn save(&self) -> $crate::__private::anyhow::Result<()> {
                    <$session>::save(self).await
                }
            }
        }
        mod undo_view {
            pub(super) struct CommandView<'a>(pub(super) &'a mut $app);
            impl $crate::command::View for CommandView<'_> {
                fn processing(&self) -> bool {
                    self.0.state.processing
                }
                fn messages(&mut self) -> &mut Vec<$crate::__private::ChatMessage> {
                    &mut self.0.state.messages
                }
                fn clear_streaming_text(&mut self) {
                    self.0
                        .state
                        .message_cache
                        .clear_streaming_text(&mut self.0.state.streaming);
                }
                fn scroll_to_bottom(&mut self) {
                    self.0.state.chat_viewport.scroll_to_bottom();
                }
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
                fn push_system_message(&mut self, message: String) {
                    $notice(self.0, message);
                }
            }
        }
        async fn handle_undo_command(app: &mut $app, session: &mut $session, rest: &str) {
            $crate::command::execute(&mut undo_view::CommandView(app), session, rest).await;
        }
    };
}
