//! Frontend write operations for the goal editor.
/// Implementation detail of [crate::declare_frontend_goal_editor].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_goal_editor_write {
    ($mode:path, $buffer:path) => {
        mod write {
            use super::Editor;
            use $buffer as FileBuffer;
            use $mode as ViewMode;
            impl $crate::Write for Editor<'_> {
                fn buffer_mut(&mut self) -> Option<&mut FileBuffer> {
                    self.app.state.editor.as_mut()
                }
                fn set_buffer(&mut self, buffer: Option<FileBuffer>) {
                    self.app.state.editor = buffer;
                }
                fn set_draft(&mut self, draft: Option<$crate::Draft>) {
                    self.app.state.goal_editor = draft;
                }
                fn reset_scroll(&mut self) {
                    self.app.state.editor_scroll = 0;
                    self.app.state.editor_hscroll = 0;
                }
                fn clear_hover(&mut self) {
                    self.app.state.editor_lsp.clear_hover();
                }
                fn clear_input(&mut self) {
                    self.app.state.clear_input();
                }
                fn show_editor(&mut self) {
                    self.app.state.set_view_mode(ViewMode::Editor);
                }
                fn show_chat(&mut self) {
                    self.app.state.set_view_mode(ViewMode::Chat);
                }
                fn status(&mut self, text: String) {
                    self.app.state.status = text;
                }
                fn redraw(&mut self) {
                    self.app.state.needs_redraw = true;
                }
            }
        }
    };
}
