//! Frontend resources for goal-command interception and deferred resume.
mod actions;
mod restart_host;
mod resume;
mod resume_host;
mod view;
/// Declare goal-command UI, runtime and session-ownership bindings.
///
/// Shared interception and deferred-resume policies keep their ordering; the
/// caller selects concrete resource types and existing command/editor services.
/// # Examples
/// See the frontend goal-input module for its concrete bindings.
#[macro_export]
macro_rules! declare_frontend_goal_input {
    ($app:path, $slot:path, $runtime:path, $registry:path, $bridge:path,
     $message:path, $kind:path, $state:path;
     $goal:path, $tasks:path, $editor:path, $resume:path $(,)?) => {
        use $app as App;
        use $slot as SessionSlot;
        use $runtime as TuiSessionHandle;
        use $registry as ProviderRegistry;
        use $bridge as TuiWorkerBridge;
        use ::std::{path::Path, sync::Arc};
        mod input_host {
            use super::{App, SessionSlot, TuiSessionHandle};
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub slot: &'a SessionSlot,
                pub runtime: &'a TuiSessionHandle,
            }
            $crate::__frontend_goal_input_actions!($state; $goal, $tasks, $editor);
            $crate::__frontend_goal_input_view!($message, $kind);
        }
        $crate::__frontend_goal_resume_host!($resume);
        $crate::__frontend_goal_restart_host!();
        $crate::__frontend_goal_resume_entries!();
        pub(crate) use resume::{drain, restart};
        /// Intercept goal/task input before borrowing the active session.
        pub(crate) async fn intercept(app: &mut App, slot: &SessionSlot, runtime: &TuiSessionHandle) -> bool {
            $crate::input::intercept(&mut input_host::Native { app, slot, runtime }).await
        }
    };
}
