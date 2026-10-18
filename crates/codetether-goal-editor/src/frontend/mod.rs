//! Frontend journal, editor-state and runtime bindings for goal editing.
mod journal;
mod read;
mod session;
mod write;
/// Declare frontend goal editing over existing state and user-journal APIs.
///
/// Read/write state, journal mutation and session restart remain separate
/// modules. Shared editor policy retains validation and save decisions.
/// # Examples
/// See the frontend goal-editor module for concrete callbacks and types.
#[macro_export]
macro_rules! declare_frontend_goal_editor {
    ($app:path, $slot:path, $runtime:path, $mode:path, $buffer:path,
     $state:path, $edit:path, $key:path; $read:path, $update:path, $restart:path $(,)?) => {
        use $app as App;
        use $slot as SessionSlot;
        use $runtime as TuiSessionHandle;
        mod host {
            use super::App;
            pub(crate) struct Editor<'a> { pub app: &'a mut App }
            $crate::__frontend_goal_editor_journal!($state, $edit; $read, $update);
            $crate::__frontend_goal_editor_read!($buffer);
            $crate::__frontend_goal_editor_write!($mode, $buffer);
            $crate::__frontend_goal_editor_session!($restart);
            pub(super) use session::Runtime;
        }
        mod open {
            use super::App;
            pub(crate) fn open(app: &mut App, id: &str) {
                $crate::open(&mut super::host::Editor { app }, id);
            }
            pub(crate) fn close(app: &mut App) {
                $crate::close(&mut super::host::Editor { app });
            }
        }
        mod paste {
            use super::App;
            pub(crate) fn paste(app: &mut App, text: &str) -> bool {
                $crate::paste(&mut super::host::Editor { app }, text)
            }
        }
        mod keys {
            use super::{App, SessionSlot, TuiSessionHandle}; use $key as KeyEvent;
            pub(crate) async fn handle(
                app: &mut App,
                slot: &SessionSlot,
                runtime: &TuiSessionHandle,
                key: KeyEvent,
            ) -> bool {
                $crate::handle(
                    &mut super::host::Runtime {
                        editor: super::host::Editor { app },
                        slot,
                        runtime,
                    },
                    key,
                )
                .await
            }
        }
    };
}
