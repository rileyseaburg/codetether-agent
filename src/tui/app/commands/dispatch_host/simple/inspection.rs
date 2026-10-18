//! inspection command execution bindings.
use super::*;
pub(super) async fn run(host: &mut Dispatch<'_>, kind: SimpleCommand) {
    let app = &mut *host.app;
    match kind {
        SimpleCommand::Audit => {
            crate::tui::audit_view::refresh_audit_snapshot(&mut app.state.audit).await;
            app.state.set_view_mode(ViewMode::Audit);
            app.state.status = "Audit — subagent activity".to_string();
        }
        SimpleCommand::Git => {
            let cwd = std::path::PathBuf::from(&app.state.cwd_display);
            app.state.git = crate::tui::git_capture::capture_git_state(&cwd);
            app.state.set_view_mode(ViewMode::Git);
            app.state.status = "Git status".to_string();
        }
        SimpleCommand::Plugin => {
            push_system_message(app, "Usage: /plugin <install|list|remove> <name>");
        }
        SimpleCommand::Keys => {
            app.state.status =
                "Protocol-first commands: /protocol /bus /context /access-mode /sandbox-mode /file /autoapply /network /autocomplete /mcp /model /sessions /import-codex /swarm /ralph /latency /symbols /settings /lsp /rlm /chat /new /undo /fork /spawn /kill /agents /agent /audit /git\nEasy aliases: /add /talk /list /remove /focus /home /say /ls /rm /main"
                    .to_string();
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}
