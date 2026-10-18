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
            let question = rest.trim();
            if question.is_empty() {
                app.state.status =
                "Usage: /ask <question> — ephemeral side question (full context, no tools, not saved)"
                    .to_string();
                push_system_message(
                    app,
                    "`/ask <question>` runs an ephemeral side query with full context but no tools, and is not saved to the session.",
                );
                return;
            }
            super::ask::run_ask(app, session, registry, question).await;
            return;
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}
