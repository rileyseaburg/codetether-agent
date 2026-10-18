#[path = "commands/mcp_host.rs"]
mod mcp_host;
use std::path::Path;
use std::sync::Arc;

#[path = "commands/a2a.rs"]
mod a2a;
#[path = "commands/fork_session.rs"]
mod fork_session;
#[path = "commands/fork_view.rs"]
mod fork_view;
#[path = "commands/go_host.rs"]
mod go_host;
#[path = "commands/goal/mod.rs"]
pub(crate) mod goal;
#[path = "commands/open_editor.rs"]
mod open_editor;
#[path = "commands/ralph_host.rs"]
mod ralph_host;
#[path = "commands/ralph_registry.rs"]
mod ralph_registry;
#[path = "commands/ralph_view.rs"]
mod ralph_view;
#[path = "commands/session_ops.rs"]
mod session_ops;
#[path = "commands/settings_command.rs"]
mod settings_command;
use codetether_settings_policy::command::Setting;
#[path = "commands/undo_session.rs"]
mod undo_session;
#[path = "commands/undo_view.rs"]
mod undo_view;
use open_editor::open_editor;
#[path = "commands/context_host.rs"]
mod context_host;
#[path = "commands/dispatch_host/mod.rs"]
mod dispatch_host;

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::session_sync::{refresh_sessions, return_to_chat};
use crate::tui::app::spawn_agent::handle_spawn_command;
use crate::tui::app::state::App;
use crate::tui::app::{codex_sessions, model_picker::open_model_picker};
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::models::ViewMode;

pub use crate::tui::app::auto_apply::{
    auto_apply_status_message, set_auto_apply_edits, toggle_auto_apply_edits,
};

fn push_system_message(app: &mut App, content: impl Into<String>) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, content.into()));
    app.state.scroll_to_bottom();
}
async fn handle_mcp_command(app: &mut App, raw: &str) {
    codetether_mcp_command::interactive::handle(&mut mcp_host::Native(app), raw).await;
}

/// Dispatch a `/context ...` command for context/RLM health status.
fn handle_context_command(app: &mut App, rest: &str) {
    codetether_context_health::command::execute(app, rest);
}

/// Undo the last `n` user turns in both the TUI and the persisted session.
///
/// `rest` may be empty (undo 1 turn) or a positive integer.
/// Each "turn" is one user message plus all assistant / tool messages that
/// followed it. Trailing system notices (e.g. a previous "Undid…" line) are
/// ignored when locating the cut point.
async fn handle_undo_command(app: &mut App, session: &mut Session, rest: &str) {
    codetether_turn_undo::command::execute(&mut undo_view::CommandView(app), session, rest).await;
}

/// Fork the current session: create a new session with a copy of the current
/// conversation (optionally truncated), switch the TUI to it, and leave the
/// original session untouched on disk.
///
/// Usage:
/// - `/fork` — fork at the current point (copy everything).
/// - `/fork N` — fork keeping only the first (total - N) turns (i.e. undo N
///   turns in the *fork* while leaving the parent intact).
async fn handle_fork_command(app: &mut App, _cwd: &Path, session: &mut Session, rest: &str) {
    codetether_session_fork::interactive::execute(&mut fork_view::CommandView(app), session, rest)
        .await;
}

/// Parse and dispatch a `/ralph ...` subcommand.
///
/// Returns `true` when the input matched a recognised subcommand (`run`,
/// `status`). Returns `false` for the bare `/ralph` (which falls through to
/// the "open monitor view" branch below).
async fn handle_ralph_subcommand(
    app: &mut App,
    cwd: &Path,
    session: &Session,
    registry: Option<&Arc<ProviderRegistry>>,
    rest: &str,
) -> bool {
    codetether_ralph_cli::interactive::handle::<ralph_host::AppHost, ProviderRegistry, _>(
        &mut ralph_view::CommandView(app),
        cwd,
        session.metadata.model.as_deref(),
        registry,
        rest,
    )
}

pub async fn handle_slash_command(
    app: &mut App,
    cwd: &std::path::Path,
    session: &mut Session,
    registry: Option<&Arc<ProviderRegistry>>,
    command: &str,
) {
    codetether_slash_command::pipeline::dispatch(
        &mut dispatch_host::Dispatch {
            app,
            cwd,
            session,
            registry,
        },
        command,
    )
    .await;
}

async fn handle_go_command(
    app: &mut App,
    session: &mut Session,
    _registry: Option<&Arc<ProviderRegistry>>,
    rest: &str,
) {
    go_host::execute(app, session, rest).await;
}

fn handle_autochat_command(app: &mut App, rest: &str) {
    codetether_autochat_core::interactive::command::handle_autochat_command::<
        super::autochat::host::Runtime,
        _,
    >(&mut app.state, rest);
}
