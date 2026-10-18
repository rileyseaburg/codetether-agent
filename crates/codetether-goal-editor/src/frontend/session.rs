//! Frontend session operations for the goal editor.
/// Implementation detail of [crate::declare_frontend_goal_editor].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_goal_editor_session {
    ($restart:path) => {
        mod session {
            use super::super::{SessionSlot, TuiSessionHandle};
            use super::Editor;
            pub(crate) struct Runtime<'a> {
                pub editor: Editor<'a>,
                pub slot: &'a SessionSlot,
                pub runtime: &'a TuiSessionHandle,
            }
            impl<'a> $crate::Session for Runtime<'a> {
                type Editor = Editor<'a>;
                fn editor(&self) -> &Self::Editor {
                    &self.editor
                }
                fn editor_mut(&mut self) -> &mut Self::Editor {
                    &mut self.editor
                }
                fn session_id(&self) -> &str {
                    self.slot.view().id()
                }
                fn restart(&mut self, id: &str) {
                    $restart(self.editor.app, id, self.runtime);
                }
            }
        }
    };
}
