//! panels actions for simple frontend commands.
/// Supporting implementation for [crate::declare_simple_commands].
#[doc(hidden)]
#[macro_export]
macro_rules! __simple_panels {
    ($dispatch:ty, $mode:path) => {
        mod panels {
            use $crate::pipeline::SimpleCommand;
            use $mode as ViewMode;
            pub(super) fn run(host: &mut $dispatch, kind: SimpleCommand) {
                let app = &mut *host.app;
                match kind {
                    SimpleCommand::Bus => {
                        crate::tui::app::state::view::transition(&mut app.state)
                            .set_view_mode(ViewMode::Bus);
                        app.state.status = "Protocol bus log".to_string();
                    }
                    SimpleCommand::Settings | SimpleCommand::Subagents => {
                        crate::tui::app::state::view::transition(&mut app.state).set_view_mode(
                            if kind == SimpleCommand::Settings {
                                ViewMode::Settings
                            } else {
                                ViewMode::Subagents
                            },
                        )
                    }
                    SimpleCommand::Lsp => crate::tui::app::state::view::transition(&mut app.state)
                        .set_view_mode(ViewMode::Lsp),
                    SimpleCommand::Rlm => crate::tui::app::state::view::transition(&mut app.state)
                        .set_view_mode(ViewMode::Rlm),
                    SimpleCommand::Latency => {
                        crate::tui::app::state::view::transition(&mut app.state)
                            .set_view_mode(ViewMode::Latency);
                        app.state.status = "Latency inspector".to_string();
                    }
                    SimpleCommand::Transport => {
                        crate::tui::app::state::view::transition(&mut app.state)
                            .set_view_mode(ViewMode::Transport);
                        app.state.status = "Transport health (TCP_INFO)".to_string();
                    }
                    SimpleCommand::Inspector => {
                        crate::tui::app::state::view::transition(&mut app.state)
                            .set_view_mode(ViewMode::Inspector);
                        app.state.status = "Inspector".to_string();
                    }
                    SimpleCommand::AuditLoop => {
                        crate::tui::app::state::view::transition(&mut app.state)
                            .set_view_mode(ViewMode::AuditLoop);
                        app.state.status = "Audit loop".to_string();
                    }
                    _ => unreachable!("command routed to the wrong native adapter"),
                }
            }
        }
    };
}
