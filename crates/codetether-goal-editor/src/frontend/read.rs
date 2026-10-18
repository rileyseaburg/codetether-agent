//! Frontend read operations for the goal editor.
/// Implementation detail of [crate::declare_frontend_goal_editor].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_goal_editor_read {
    ($buffer:path) => {
        mod read {
            use super::Editor;
            use $buffer as FileBuffer;
            impl $crate::Read for Editor<'_> {
                type Journal = super::journal::Native;
                fn draft(&self) -> Option<&$crate::Draft> {
                    self.app.state.goal_editor.as_ref()
                }
                fn buffer(&self) -> Option<&FileBuffer> {
                    self.app.state.editor.as_ref()
                }
                fn approval_open(&self) -> bool {
                    self.app.state.approval_edit.is_some()
                }
            }
        }
    };
}
