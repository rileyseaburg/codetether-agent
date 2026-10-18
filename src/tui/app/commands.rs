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
#[path = "commands/session_ops.rs"]
mod session_ops;
codetether_settings_policy::declare_setting_commands!(
    crate::tui::app::state::App,
    crate::session::Session,
    crate::tui::app::auto_apply::set_auto_apply_edits,
    crate::tui::app::settings::set_network_access,
    crate::tui::app::settings::set_slash_autocomplete,
);
use codetether_settings_policy::command::Setting;
use open_editor::open_editor;
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
codetether_mcp_command::declare_frontend_command!(
    crate::tui::app::state::App,
    crate::tui::app::commands::push_system_message,
);
codetether_context_health::declare_context_command!(
    crate::tui::app::state::App,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);

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

codetether_ralph_cli::declare_frontend_ralph!(
    crate::tui::app::state::App,
    crate::session::Session,
    crate::ralph::RalphLoop,
    crate::provider::ProviderRegistry,
    crate::tui::models::ViewMode,
    crate::tui::app::commands::push_system_message,
);

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
