//! Chat prompt submission for the TUI.
//!
//! Validates the input, dispatches `!shell` and slash commands, pushes
//! user messages, and delegates to
//! [`super::chat_submit_dispatch::dispatch_prompt`].
//!
//! Mid-stream behavior: while a local turn is in flight, Enter injects text
//! and images before its next model step.
//!
//! [`handle_enter_chat`] is the Enter-key entry point. It gives goal-answer
//! commands, goal answers, and approval feedback priority over ordinary chat
//! submission. If none consumes the input, it captures the prompt, attempts
//! managed-agent routing, and resolves mention routing before checking
//! continuation and slash commands.
//!
//! Each handler that consumes the submission stops further processing. Only
//! input left unhandled by those routes reaches active-turn steering or the
//! normal submission path in [`super::chat_submit_finish::finish_submit`].

mod host;
use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[cfg(test)]
#[path = "chat_submit_queue_tests.rs"]
mod queue_tests;

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
    codetether_chat_submit::submit(&mut host::Native {
        app,
        cwd,
        slot,
        registry,
        worker_bridge,
        runtime,
    })
    .await;
}
