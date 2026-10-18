//! execution command execution bindings.
use super::*;
pub(super) async fn run(host: &mut Dispatch<'_>, kind: LeadingCommand, rest: &str) {
    let app = &mut *host.app;
    let session = &mut *host.session;
    let registry = host.registry;
    match kind {
        codetether_slash_command::routing::LeadingCommand::Go => {
            handle_go_command(app, session, registry, rest).await;
            return;
        }
        codetether_slash_command::routing::LeadingCommand::Ask => {
            crate::tui::app::ask::run_command(app, session, registry, rest).await;
            return;
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}
