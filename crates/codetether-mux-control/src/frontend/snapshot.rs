//! Borrowed frontend state projection for mux runtime reporting.
/// Declare the borrowed mux view over existing frontend and session fields.
///
/// # Examples
/// ```text
/// codetether_mux_control::declare_frontend_runtime_view!(App, SessionView, RuntimeView);
/// ```
#[macro_export]
macro_rules! declare_frontend_runtime_view {
    ($app:path, $session:path, $view:path $(,)?) => {
        use $app as App;
        use $session as SessionView;
        use $view as RuntimeView;
        pub(super) fn view<'a>(app: &'a App, session: &'a SessionView) -> RuntimeView<'a> {
            RuntimeView {
                session_id: &session.id,
                title: session.title.as_deref(),
                processing: app.state.processing,
                message_count: session.message_count,
                current_tool: app.state.tool_timing.pending_tool_name.as_deref(),
                input: &app.state.composer.input,
                watchdog_pending: app.state.watchdog.notification.is_some(),
                restarts: app.state.watchdog.restart_count,
                status: &app.state.status,
                principal: &session.principal,
            }
        }
    };
}
