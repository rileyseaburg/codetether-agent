//! Frontend state, effect and fallback routing for modifier shortcuts.
mod input;
mod routing;
mod state;
mod view;
/// Declare modified-key dispatch using frontend shortcut handlers.
///
/// Shared policy owns modifier priority; the caller selects state enums, voice
/// behavior and existing action callbacks without changing dispatch order.
/// # Examples
/// See the frontend keyboard module for concrete callback selection.
#[macro_export]
macro_rules! declare_frontend_modified_keys {
    ($app:path, $runtime:path, $key:path, $mode:path, $layout:path;
     $question:path, $reply:path, $transcript:path, $voice:path, $paste:path, $cancel:path;
     $alt:path, $approval:path, $ctrl_c:path, $fallback:path $(,)?) => {
        use $app as App;
        use $runtime as TuiSessionHandle;
        use $key as KeyEvent;
        use ::std::path::Path;
        mod host {
            use super::{App, Path, TuiSessionHandle};
            use $crate::modified::{Action, Effects};
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub runtime: &'a TuiSessionHandle,
            }
            impl Effects for Native<'_> {
                fn effect(&mut self, action: Action) {
                    if input::apply(self.app, &action) || view::apply(self.app, &action) {
                        return;
                    }
                    unreachable!("unmapped shortcut action");
                }
            }
            $crate::__frontend_modified_input!($question, $reply, $transcript, $voice, $paste, $cancel);
            $crate::__frontend_modified_view!();
            $crate::__frontend_modified_state!($mode, $layout);
            $crate::__frontend_modified_routing!($key; $alt, $approval, $ctrl_c, $fallback);
        }
        pub(super) fn handle_ctrl_key(
            app: &mut App,
            cwd: &Path,
            runtime: &TuiSessionHandle,
            key: KeyEvent,
        ) -> Option<anyhow::Result<bool>> {
            $crate::modified::dispatch(&mut host::Native { app, cwd, runtime }, key)
        }
    };
}
