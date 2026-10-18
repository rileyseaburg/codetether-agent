//! Native bindings for shared interactive slash dispatch.
use super::*;
use codetether_slash_command::pipeline::TailCommand;
codetether_slash_command::declare_leading_commands!(
    crate::tui::app::commands::dispatch_host::Dispatch<'_>,
    codetether_image::interactive::Presentation,
    codetether_settings_policy::command::Setting;
    crate::tui::app::input::attach_image_command,
    crate::tui::app::file_share::command,
    crate::tui::app::commands::open_editor;
    crate::tui::app::commands::handle_context_command,
    crate::tui::app::commands::handle_mcp_command;
    crate::tui::app::commands::handle_go_command,
    crate::tui::app::ask::run_command;
    crate::tui::app::commands::settings_command::execute,
);
codetether_slash_command::declare_simple_commands!(
    crate::tui::app::commands::dispatch_host::Dispatch<'_>,
    crate::tui::models::ViewMode,
    crate::tui::ui::webview::layout_mode::ChatLayoutMode;
    crate::tui::app::session_sync::refresh_sessions,
    crate::tui::app::codex_sessions::import_workspace_sessions,
    crate::tui::app::model_picker::open_model_picker,
    crate::tui::app::session_sync::create_new_session;
    crate::tui::swarm_run::open_swarm_monitor,
    crate::tui::app::session_sync::return_to_chat;
    crate::tui::audit_view::refresh_audit_snapshot,
    crate::tui::git_capture::capture_git_state,
    crate::tui::app::commands::push_system_message;
    codetether_cognition_core::palace::command::recall,
);
codetether_cognition_core::declare_recall_view!(
    crate::tui::app::state::App,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);
mod tail;
codetether_slash_command::declare_dispatch_host!(
    crate::tui::app::state::App, crate::session::Session, crate::provider::ProviderRegistry;
    crate::tui::app::commands::dispatch_host::leading::run,
    crate::tui::app::commands::dispatch_host::simple::run,
    crate::tui::app::commands::dispatch_host::tail::run;
    crate::tui::app::commands::handle_ralph_subcommand,
    crate::tui::swarm_run::handle_swarm_command;
    crate::tui::forage_run::handle_forage_command,
    crate::tui::app::commands::goal::handle,
    crate::tui::app::commands::a2a::handle,
    crate::tui::app::commands::handle_undo_command,
    crate::tui::app::detach::handle_detach_command,
    crate::tui::app::commands::handle_fork_command,
);
