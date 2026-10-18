//! diagnostics command execution bindings.
use super::*;
pub(super) async fn run(host: &mut Dispatch<'_>, kind: LeadingCommand, rest: &str) {
    let app = &mut *host.app;
    match kind {
        codetether_slash_command::routing::LeadingCommand::Context => {
            handle_context_command(app, rest);
            return;
        }
        codetether_slash_command::routing::LeadingCommand::Mcp => {
            handle_mcp_command(app, rest).await;
            return;
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}
