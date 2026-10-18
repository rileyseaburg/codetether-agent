//! Frontend editor state and external key-action effects.
/// Implementation detail of [crate::declare_frontend_editor_keys].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_editor_keys_host {
    ($mode:path; $approval:path, $finder:path) => {
        mod host {
            use super::{App, Path};
            use $crate::{EditorInput, FileBuffer};
            use $mode as ViewMode;
            pub(super) struct Native<'a>(pub &'a mut App);
            impl $crate::interactive::KeyHost for Native<'_> {
                fn in_editor(&self) -> bool {
                    self.0.state.view_mode == ViewMode::Editor
                }
                fn approval_editing(&self) -> bool {
                    self.0.state.approval_edit.is_some()
                }
                fn handle_approval_action(&mut self, action: &EditorInput) -> bool {
                    $approval(self.0, action)
                }
                fn editor(&mut self) -> &mut Option<FileBuffer> {
                    &mut self.0.state.editor.buffer
                }
                fn show_chat(&mut self) {
                    self.0.state.view_mode = ViewMode::Chat;
                }
                fn reset_scroll(&mut self) {
                    self.0.state.editor.scroll = 0;
                    self.0.state.editor.hscroll = 0;
                }
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
                fn redraw(&mut self) {
                    self.0.state.needs_redraw = true;
                }
                fn open_finder(&mut self, cwd: &Path) {
                    $finder(self.0, cwd);
                }
            }
        }
    };
}
