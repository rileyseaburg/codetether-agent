//! Native bindings for shared interactive slash dispatch.
use super::*;
use codetether_slash_command::pipeline::{SimpleCommand, TailCommand};
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
mod simple;
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
