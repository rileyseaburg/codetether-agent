//! Frontend resource bindings for steering.
mod input;
mod view;
/// Declare frontend steering using the existing session runtime and UI state.
///
/// Concrete frontend types remain selected by the caller. The shared policy
/// retains sequencing and session/attachment ownership transitions.
/// # Examples
/// See the frontend chat_steer module for concrete type and service selection.
#[macro_export]
macro_rules! declare_frontend_steering {
    ($app:path, $runtime:path, $image:path, $steering:path, $message:path; $expand:path, $queue:path $(,)?) => {
        use $app as App;
        use $runtime as TuiSessionHandle;
        mod host {
            use super::{App, TuiSessionHandle};
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub runtime: &'a TuiSessionHandle,
            }
            $crate::__frontend_steering_input!($image, $steering, $expand, $queue);
            $crate::__frontend_steering_view!($message);
        }
        pub(super) fn submit(app: &mut App, runtime: &TuiSessionHandle, prompt: &str) {
            $crate::steering::submit(&mut host::Native { app, runtime }, prompt);
        }
    };
}
