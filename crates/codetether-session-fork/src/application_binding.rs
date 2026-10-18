//! Frontend session and view bindings for fork commands.
/// Declare fork command adapters using existing frontend and session state.
///
/// The notice callback retains application message presentation. Session history,
/// metadata and persistence operations remain on the selected session type.
/// # Examples
/// ```text
/// codetether_session_fork::declare_fork_command!(App, Session, ViewMode, push_notice);
/// ```
#[macro_export]
macro_rules! declare_fork_command {
    ($app:ty, $session:ty, $mode:path, $notice:path $(,)?) => {
        mod fork_session {
            impl $crate::interactive::Session for $session {
                fn pages(&self) -> &[$crate::__private::PageKind] {
                    &self.pages
                }
                fn install_history(
                    &mut self,
                    messages: Vec<$crate::__private::Message>,
                    pages: Vec<$crate::__private::PageKind>,
                ) {
                    self.messages = messages.into();
                    self.pages = pages.into();
                }
                fn metadata_mut(&mut self) -> &mut $crate::__private::SessionMetadata {
                    &mut self.metadata
                }
                fn attach_global_bus_if_missing(&mut self) {
                    <$session>::attach_global_bus_if_missing(self);
                }
            }
        }
        mod fork_view {
            use $mode as ViewMode;
            pub(super) struct CommandView<'a>(pub(super) &'a mut $app);
            impl $crate::interactive::View for CommandView<'_> {
                fn processing(&self) -> bool {
                    self.0.state.processing
                }
                fn messages(&self) -> &[$crate::__private::ChatMessage] {
                    &self.0.state.messages
                }
                fn set_messages(&mut self, messages: Vec<$crate::__private::ChatMessage>) {
                    self.0.state.messages = messages;
                }
                fn set_session_id(&mut self, id: String) {
                    self.0.state.session_id = Some(id);
                }
                fn clear_streaming_text(&mut self) {
                    self.0.state.clear_streaming_text();
                }
                fn clear_request_timing(&mut self) {
                    self.0.state.clear_request_timing();
                }
                fn scroll_to_bottom(&mut self) {
                    self.0.state.scroll_to_bottom();
                }
                fn show_chat(&mut self) {
                    self.0.state.set_view_mode(ViewMode::Chat);
                }
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
                fn push_system_message(&mut self, message: String) {
                    $notice(self.0, message);
                }
            }
        }
        async fn handle_fork_command(
            app: &mut $app,
            _cwd: &::std::path::Path,
            session: &mut $session,
            rest: &str,
        ) {
            $crate::interactive::execute(&mut fork_view::CommandView(app), session, rest).await;
        }
    };
}
