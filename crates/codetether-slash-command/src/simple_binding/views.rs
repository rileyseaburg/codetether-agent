//! views actions for simple frontend commands.
/// Supporting implementation for [crate::declare_simple_commands].
#[doc(hidden)]
#[macro_export]
macro_rules! __simple_views {
    ($dispatch:ty, $mode:path, $layout:path, $swarm:path, $chat:path) => {
        mod views {
            use $crate::pipeline::SimpleCommand;
            use $layout as ChatLayoutMode;
            use $mode as ViewMode;
            pub(super) fn run(host: &mut $dispatch, kind: SimpleCommand) {
                let app = &mut *host.app;
                match kind {
                    SimpleCommand::Help => {
                        app.state.show_help = true;
                        app.state.help_scroll.offset = 0;
                        app.state.status = "Help".to_string();
                    }
                    SimpleCommand::Swarm => $swarm(app),
                    SimpleCommand::Ralph => {
                        app.state
                            .ralph
                            .mark_active(app.state.cwd_display.clone(), "TUI Ralph monitor");
                        app.state.set_view_mode(ViewMode::Ralph);
                    }
                    SimpleCommand::Chat => $chat(app),
                    SimpleCommand::Webview => {
                        app.state.chat_layout_mode = ChatLayoutMode::Webview;
                        app.state.status = "Layout: Webview".to_string();
                    }
                    SimpleCommand::Classic => {
                        app.state.chat_layout_mode = ChatLayoutMode::Classic;
                        app.state.status = "Layout: Classic".to_string();
                    }
                    SimpleCommand::Symbols => {
                        app.state.symbol_search.open();
                        app.state.status = "Symbol search".to_string();
                    }
                    _ => unreachable!("command routed to the wrong native adapter"),
                }
            }
        }
    };
}
