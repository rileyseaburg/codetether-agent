//! Native bindings for shared interactive slash dispatch.
use super::*;
use codetether_slash_command::{
    pipeline::{SimpleCommand, TailCommand},
    routing::LeadingCommand,
};
mod leading;
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
