//! settings command execution bindings.
use super::*;
pub(super) async fn run(host: &mut Dispatch<'_>, kind: LeadingCommand, rest: &str) {
    let app = &mut *host.app;
    let session = &mut *host.session;
    match kind {
        codetether_slash_command::routing::LeadingCommand::AutoApply => {
            settings_command::execute(app, session, Setting::AutoApply, rest).await;
            return;
        }
        codetether_slash_command::routing::LeadingCommand::Network => {
            settings_command::execute(app, session, Setting::Network, rest).await;
            return;
        }
        codetether_slash_command::routing::LeadingCommand::Autocomplete => {
            settings_command::execute(app, session, Setting::Autocomplete, rest).await;
            return;
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}
