//! Frontend approval preview and editor-state effects.
/// Implementation detail of [crate::declare_frontend_approval_keys].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_approval_keys_host {
    ($mode:path, $buffer:path, $session:path, $command:path) => {
        mod host {
            use super::App;
            use $buffer as FileBuffer;
            use $crate::input::Preview;
            use $mode as ViewMode;
            use $session as ApprovalEditSession;
            pub(super) struct Native<'a>(pub &'a mut App);
            impl $crate::interactive::Host for Native<'_> {
                fn preview(&mut self) -> Preview<'_> {
                    Preview {
                        chat: self.0.state.view_mode == ViewMode::Chat,
                        editing: self.0.state.approval_edit.is_some(),
                        help: self.0.state.show_help,
                        input: &self.0.state.input,
                        scroll: &mut self.0.state.approval_preview_scroll,
                    }
                }
                fn run_command(&mut self, command: &str) -> bool {
                    $command(self.0, command)
                }
                fn editor(&mut self) -> &mut Option<FileBuffer> {
                    &mut self.0.state.editor.buffer
                }
                fn session(&mut self) -> &mut Option<ApprovalEditSession> {
                    &mut self.0.state.approval_edit
                }
                fn reset_editor_scroll(&mut self) {
                    self.0.state.editor.scroll = 0;
                    self.0.state.editor.hscroll = 0;
                }
                fn show_editor(&mut self) {
                    self.0.state.set_view_mode(ViewMode::Editor);
                }
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
            }
        }
    };
}
