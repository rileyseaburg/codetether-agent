#[path = "commands/mcp_host.rs"]
mod mcp_host;
use std::path::Path;
use std::sync::Arc;

#[path = "commands/a2a.rs"]
mod a2a;
#[path = "commands/go_host.rs"]
mod go_host;
#[path = "commands/goal/mod.rs"]
pub(crate) mod goal;
#[path = "commands/open_editor.rs"]
mod open_editor;
#[path = "commands/ralph_host.rs"]
mod ralph_host;
#[path = "commands/ralph_view.rs"]
mod ralph_view;
#[path = "commands/session_ops.rs"]
mod session_ops;
#[path = "commands/settings_command.rs"]
mod settings_command;
use codetether_settings_policy::command::Setting;
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

codetether_turn_undo::declare_undo_command!(
    crate::tui::app::state::App,
    crate::session::Session,
    crate::tui::app::commands::push_system_message,
);
codetether_session_fork::declare_fork_command!(
    crate::tui::app::state::App,
    crate::session::Session,
    crate::tui::models::ViewMode,
    crate::tui::app::commands::push_system_message,
);

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
