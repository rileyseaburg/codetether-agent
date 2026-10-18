//! Frontend type and callback selection for chat command intake.
mod host;
mod session;
/// Declare shell/approval/mux intake and optional live-session command handling.
///
/// The shared intake policy retains route order. The frontend retains its
/// concrete session slot and the borrow used for parity and ordinary commands.
/// # Examples
/// See the frontend chat-submit slash module for concrete callback selection.
#[macro_export]
macro_rules! declare_frontend_command_intake {
    ($app:path, $slot:path, $registry:path, $session:path;
     $shell:path, $approval:path, $mux:path, $session_free:path;
     $parity:path, $command:path $(,)?) => {
        use ::std::{path::Path, sync::Arc};
        use $app as App;
        use $registry as ProviderRegistry;
        use $slot as SessionSlot;
        mod host {
            use super::{App, Arc, Path, ProviderRegistry, SessionSlot};
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub slot: &'a mut SessionSlot,
                pub registry: &'a Option<Arc<ProviderRegistry>>,
            }
            $crate::__frontend_command_intake!($shell, $approval, $mux, $session_free);
            $crate::__frontend_command_session!($session, $parity, $command);
        }
        /// Route a captured prompt through command intake.
        /// # Arguments
        /// Receives frontend/session context, provider configuration and prompt.
        /// # Returns
        /// True when command intake handled the prompt.
        /// # Examples
        /// See the frontend chat-routing host.
        pub(super) async fn run(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            prompt: &str,
        ) -> bool {
            $crate::intake::run(
                &mut host::Native {
                    app,
                    cwd,
                    slot,
                    registry,
                },
                prompt,
            )
            .await
        }
    };
}
