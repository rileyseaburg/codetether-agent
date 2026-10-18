//! Frontend buffer and preview state for approval-backed editing.
/// Declare the frontend approval editor host and key entry point.
/// # Examples
/// See the frontend approval-editor module for concrete state and editor types.
#[macro_export]
macro_rules! declare_frontend_approval_editor {
    ($visibility:vis; $app:path, $mode:path, $buffer:path, $session:path, $action:path $(,)?) => {
        mod host {
            use $app as App;
            use $mode as ViewMode;
            use $buffer as FileBuffer;
            use $session as ApprovalEditSession;
            pub(super) struct Native<'a>(pub &'a mut App);
            impl $crate::interactive::Host for Native<'_> {
                fn editor(&mut self) -> &mut Option<FileBuffer> {
                    &mut self.0.state.editor
                }
                fn session(&mut self) -> &mut Option<ApprovalEditSession> {
                    &mut self.0.state.approval_edit
                }
                fn reset_scroll(&mut self) {
                    self.0.state.editor_scroll = 0;
                    self.0.state.editor_hscroll = 0;
                }
                fn show_chat(&mut self) {
                    self.0.state.set_view_mode(ViewMode::Chat);
                }
                fn reset_preview_scroll(&mut self) {
                    self.0.state.approval_preview_scroll = 0;
                }
                fn set_waiting(&mut self, waiting: bool) {
                    self.0.state.approval_waiting = waiting;
                }
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
                fn redraw(&mut self) {
                    self.0.state.needs_redraw = true;
                }
            }
        }
        mod key {
            use $app as App;
            use $action as EditorInput;
            $visibility fn handle(app: &mut App, action: &EditorInput) -> bool {
                $crate::interactive::handle(&mut super::host::Native(app), action)
            }
        }
    };
}
