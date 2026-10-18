//! panels command execution bindings.
use super::*;
pub(super) fn run(host: &mut Dispatch<'_>, kind: SimpleCommand) {
    let app = &mut *host.app;
    match kind {
        SimpleCommand::Bus => {
            app.state.set_view_mode(ViewMode::Bus);
            app.state.status = "Protocol bus log".to_string();
        }
        SimpleCommand::Settings | SimpleCommand::Subagents => {
            app.state.set_view_mode(if kind == SimpleCommand::Settings {
                ViewMode::Settings
            } else {
                ViewMode::Subagents
            })
        }
        SimpleCommand::Lsp => app.state.set_view_mode(ViewMode::Lsp),
        SimpleCommand::Rlm => app.state.set_view_mode(ViewMode::Rlm),
        SimpleCommand::Latency => {
            app.state.set_view_mode(ViewMode::Latency);
            app.state.status = "Latency inspector".to_string();
        }
        SimpleCommand::Transport => {
            app.state.set_view_mode(ViewMode::Transport);
            app.state.status = "Transport health (TCP_INFO)".to_string();
        }
        SimpleCommand::Inspector => {
            app.state.set_view_mode(ViewMode::Inspector);
            app.state.status = "Inspector".to_string();
        }
        SimpleCommand::AuditLoop => {
            app.state.set_view_mode(ViewMode::AuditLoop);
            app.state.status = "Audit loop".to_string();
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}
