//! views command execution bindings.
use super::*;
pub(super) fn run(host: &mut Dispatch<'_>, kind: SimpleCommand) {
    let app = &mut *host.app;
    match kind {
        SimpleCommand::Help => {
            app.state.show_help = true;
            app.state.help_scroll.offset = 0;
            app.state.status = "Help".to_string();
        }
        SimpleCommand::Swarm => crate::tui::swarm_run::open_swarm_monitor(app),
        SimpleCommand::Ralph => {
            app.state
                .ralph
                .mark_active(app.state.cwd_display.clone(), "TUI Ralph monitor");
            app.state.set_view_mode(ViewMode::Ralph);
        }
        SimpleCommand::Chat => return_to_chat(app),
        SimpleCommand::Webview => {
            app.state.chat_layout_mode =
                crate::tui::ui::webview::layout_mode::ChatLayoutMode::Webview;
            app.state.status = "Layout: Webview".to_string();
        }
        SimpleCommand::Classic => {
            app.state.chat_layout_mode =
                crate::tui::ui::webview::layout_mode::ChatLayoutMode::Classic;
            app.state.status = "Layout: Classic".to_string();
        }
        SimpleCommand::Symbols => {
            app.state.symbol_search.open();
            app.state.status = "Symbol search".to_string();
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}
