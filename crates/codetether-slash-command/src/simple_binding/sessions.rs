//! sessions actions for simple frontend commands.
/// Supporting implementation for [crate::declare_simple_commands].
#[doc(hidden)]
#[macro_export]
macro_rules! __simple_sessions {
    ($dispatch:ty, $mode:path, $refresh:path, $import:path, $model:path, $new:path) => {
        mod sessions {
            use $crate::pipeline::SimpleCommand;
            use $mode as ViewMode;
            pub(super) async fn run(host: &mut $dispatch, kind: SimpleCommand) {
                let app = &mut *host.app;
                let cwd = host.cwd;
                let session = &mut *host.session;
                let registry = host.registry;
                match kind {
                    SimpleCommand::Sessions => {
                        $refresh(app, cwd).await;
                        app.state.session_picker.clear_session_filter();
                        crate::tui::app::state::view::transition(&mut app.state)
                            .set_view_mode(ViewMode::Sessions);
                        app.state.status = "Session picker".to_string();
                    }
                    SimpleCommand::ImportCodex => {
                        $import(app, cwd).await;
                    }
                    SimpleCommand::Model => $model(app, session, registry).await,
                    SimpleCommand::New => $new(app, cwd, session).await,
                    _ => unreachable!("command routed to the wrong native adapter"),
                }
            }
        }
    };
}
