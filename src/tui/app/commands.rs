#[path = "commands/goal/mod.rs"]
pub(crate) mod goal;
codetether_settings_policy::declare_setting_commands!(
    crate::tui::app::state::App,
    crate::session::Session,
    crate::tui::app::auto_apply::set_auto_apply_edits,
    crate::tui::app::settings::set_network_access,
    crate::tui::app::settings::set_slash_autocomplete,
);
#[path = "commands/dispatch_host/mod.rs"]
mod dispatch_host;

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

codetether_cognition_core::declare_recall_view!(
    crate::tui::app::state::App,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);

pub use crate::tui::app::auto_apply::{
    auto_apply_status_message, set_auto_apply_edits, toggle_auto_apply_edits,
};

fn push_system_message(app: &mut App, content: impl Into<String>) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, content.into()));
    app.state.chat_viewport.scroll_to_bottom();
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

codetether_slash_command::declare_slash_entry!(
    crate::tui::app::state::App,
    crate::session::Session,
    crate::provider::ProviderRegistry,
);

codetether_relay_okr::declare_go_command!(
    crate::tui::app::state::App,
    crate::session::Session,
    crate::provider::ProviderRegistry,
    crate::autochat::model_rotation::resolve_provider_for_model_autochat,
    crate::tui::app::commands::push_system_message,
    crate::tui::constants::AUTOCHAT_DEFAULT_AGENTS,
);

fn handle_autochat_command(app: &mut App, rest: &str) {
    codetether_autochat_core::interactive::command::handle_autochat_command::<
        super::autochat::host::Runtime,
        _,
    >(&mut app.state, rest);
}
