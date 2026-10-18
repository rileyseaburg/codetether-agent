//! Frontend resource bindings for handoff.
mod preparation;
mod transfer;
/// Declare frontend handoff using the existing session runtime and UI state.
///
/// Concrete frontend types remain selected by the caller. The shared policy
/// retains sequencing and session/attachment ownership transitions.
/// # Examples
/// See the frontend chat_spawn module for concrete type and service selection.
#[macro_export]
macro_rules! declare_frontend_handoff {
    ($app:path, $slot:path, $registry:path, $runtime:path, $session:path, $request:path, $image:path $(,)?) => {
        use ::std::{path::Path, sync::Arc};
        use $app as App;
        use $image as ImageAttachment;
        use $registry as ProviderRegistry;
        use $runtime as TuiSessionHandle;
        use $slot as SessionSlot;
        mod host {
            use super::{App, Arc, ProviderRegistry, SessionSlot, TuiSessionHandle};
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub slot: &'a mut SessionSlot,
                pub registry: &'a Arc<ProviderRegistry>,
                pub runtime: &'a TuiSessionHandle,
            }
            $crate::__frontend_handoff_preparation!($image, $request);
            $crate::__frontend_handoff_transfer!($session, $request);
        }
        /// Spawn the provider task with optional worktree isolation.
        ///
        /// Sets up the worktree (if `use_worktree` is enabled),
        /// clones the session, and spawns a background task.
        ///
        /// The active session is taken from `slot` and submitted to `runtime` as a
        /// [`PromptRequest`](crate::tui::app::session_runtime::PromptRequest). While the runtime owns the session, the TUI treats the
        /// slot as busy. If submission fails, the session is restored immediately so
        /// the user can retry without losing conversation state.
        ///
        /// # Parameters
        ///
        /// * `app` - TUI application state. The function reads `auto_apply_edits` and
        ///   `use_worktree` from `app.state`, and writes a status message when the
        ///   session cannot be submitted.
        /// * `cwd` - Current working directory used as the worktree root when worktree
        ///   isolation is enabled.
        /// * `slot` - Holder for the active session. The session must be idle and
        ///   available to be moved into the runtime.
        /// * `registry` - Shared provider registry used by the runtime to resolve the
        ///   selected model provider.
        /// * `prompt` - Prompt text entered by the user.
        /// * `pending_images` - Image attachments to send with the prompt.
        /// * `runtime` - Session runtime handle that accepts prompt requests for
        ///   background execution.
        ///
        /// # Side Effects
        ///
        /// Updates session metadata, may remove the session from `slot`, submits work
        /// to the runtime, and updates `app.state.status` when the session is already
        /// running or the runtime is unavailable.
        ///
        /// # Errors
        ///
        /// This function does not return an error. Runtime submission failure is handled
        /// by restoring the session into `slot` and setting a user-visible status
        /// message.
        ///
        /// # Preconditions
        ///
        /// `slot` should contain an idle session. If the slot cannot be borrowed, the
        /// function assumes a prompt is already running and returns without submitting
        /// new work.
        pub(super) async fn spawn_provider_task(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Arc<ProviderRegistry>,
            prompt: &str,
            pending_images: Vec<ImageAttachment>,
            runtime: &TuiSessionHandle,
        ) {
            $crate::handoff::prepare(
                &mut host::Native {
                    app,
                    slot,
                    registry,
                    runtime,
                },
                cwd,
                prompt,
                pending_images,
            )
            .await;
        }
    };
}
