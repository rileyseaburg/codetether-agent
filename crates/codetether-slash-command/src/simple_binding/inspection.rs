//! inspection actions for simple frontend commands.
/// Supporting implementation for [crate::declare_simple_commands].
#[doc(hidden)]
#[macro_export]
macro_rules! __simple_inspection {
    ($dispatch:ty, $mode:path, $audit:path, $git:path, $notice:path) => {
        mod inspection {
            use $crate::pipeline::SimpleCommand;
            use $mode as ViewMode;
pub(super) async fn run(host: &mut $dispatch, kind: SimpleCommand) {
    let app = &mut *host.app;
    match kind {
        SimpleCommand::Audit => {
            $audit(&mut app.state.audit).await;
            app.state.set_view_mode(ViewMode::Audit);
            app.state.status = "Audit — subagent activity".to_string();
        }
        SimpleCommand::Git => {
            let cwd = std::path::PathBuf::from(&app.state.cwd_display);
            app.state.git = $git(&cwd);
            app.state.set_view_mode(ViewMode::Git);
            app.state.status = "Git status".to_string();
        }
        SimpleCommand::Plugin => {
            $notice(app, "Usage: /plugin <install|list|remove> <name>");
        }
        SimpleCommand::Keys => {
            app.state.status =
                "Protocol-first commands: /protocol /bus /context /access-mode /sandbox-mode /file /autoapply /network /autocomplete /mcp /model /sessions /import-codex /swarm /ralph /latency /symbols /settings /lsp /rlm /chat /new /undo /fork /spawn /kill /agents /agent /audit /git\nEasy aliases: /add /talk /list /remove /focus /home /say /ls /rm /main"
                    .to_string();
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}

        }
    };
}
