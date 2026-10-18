//! Frontend state and handler selection for complete chat submission.
mod intercept;
mod routing;
mod submission;
/// Declare the frontend's chat submission entry and native handler groups.
///
/// Input interception, routing and final submission remain separate modules;
/// [crate::submit] retains the ordering and early-return policy.
/// # Examples
/// See the frontend chat submission module for the concrete service selection.
#[macro_export]
macro_rules! declare_frontend_chat_submit {
    ($app:path, $slot:path, $registry:path, $bridge:path, $runtime:path;
     $goal_command:path, $goal_answer:path, $feedback:path;
     $capture:path, $managed:path, $mentions:path, $continuation:path, $slash:path;
     $steer:path, $finish:path $(,)?) => {
        use ::std::{path::Path, sync::Arc};
        use $app as App;
        use $bridge as TuiWorkerBridge;
        use $registry as ProviderRegistry;
        use $runtime as TuiSessionHandle;
        use $slot as SessionSlot;
        mod host {
            use super::{
                App, Arc, Path, ProviderRegistry, SessionSlot, TuiSessionHandle, TuiWorkerBridge,
            };
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub slot: &'a mut SessionSlot,
                pub registry: &'a Option<Arc<ProviderRegistry>>,
                pub worker_bridge: &'a Option<TuiWorkerBridge>,
                pub runtime: &'a TuiSessionHandle,
            }
            $crate::__frontend_chat_intercept!($goal_command, $goal_answer, $feedback);
            $crate::__frontend_chat_routing!($capture, $managed, $mentions, $continuation, $slash);
            $crate::__frontend_chat_submission!($steer, $finish);
        }
        /// Submit the chat input to the provider.
        ///
        /// Handles `!shell` and slash commands, pushes user/image messages,
        /// then delegates prompt preparation and dispatch. While a previous
        /// request is still in flight, input steers that active prompt run.
        ///
        /// # Arguments
        ///
        /// * `app` — Mutable TUI application state containing the current input,
        ///   selected spawned agent, and processing status. Submission handlers use
        ///   this state to consume input and update the interface.
        /// * `cwd` — Working directory passed to continuation, slash-command, and
        ///   normal submission handlers.
        /// * `slot` — Mutable session slot used by goal-answer handlers and by
        ///   commands or submissions that operate on the current session.
        /// * `registry` — Optional shared provider registry forwarded to handlers
        ///   that may need provider access. This function does not require it to be
        ///   present before routing input.
        /// * `worker_bridge` — Optional worker bridge forwarded to continuation and
        ///   normal submission handlers.
        /// * `runtime` — Session runtime handle used by goal-answer handling,
        ///   continuation, active-turn steering, and normal submission.
        ///
        /// # Returns
        ///
        /// Returns `()` after a handler consumes the submission or the normal
        /// submission handler returns. Returning does not itself indicate that a
        /// provider response has finished.
        ///
        /// # Routing and side effects
        ///
        /// Goal-answer and approval-feedback interception happens before prompt
        /// capture. Managed-agent routing receives the captured prompt before mention
        /// routing is applied. Continuation and slash commands are checked before
        /// `app.state.processing`, so they retain priority even during an active turn.
        ///
        /// If the application is still processing after those checks, the remaining
        /// prompt is submitted to the steering handler rather than starting the normal
        /// submission path. Otherwise, prompt submission is delegated to
        /// [`super::chat_submit_finish::finish_submit`]. Input capture and the selected
        /// handlers may update application or session state and initiate work.
        ///
        /// # Error handling
        ///
        /// This function exposes no `Result` or separate error value. Input rejection,
        /// unavailable dependencies, and operational error reporting are left to the
        /// delegated handlers.
        pub(super) async fn handle_enter_chat(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            worker_bridge: &Option<TuiWorkerBridge>,
            runtime: &TuiSessionHandle,
        ) {
            $crate::submit(&mut host::Native {
                app,
                cwd,
                slot,
                registry,
                worker_bridge,
                runtime,
            })
            .await;
        }
    };
}
