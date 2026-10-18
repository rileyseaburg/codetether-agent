//! Frontend type selection for the borrowed mux runtime view.
codetether_mux_control::declare_frontend_runtime_view!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionView,
    codetether_mux_model::RuntimeView,
);
#[cfg(test)]
fn unchanged(last: &crate::mux::MuxRuntimeStatus, app: &App, session: &SessionView) -> bool {
    view(app, session).unchanged(last)
}
#[cfg(test)]
fn build(app: &App, session: &SessionView) -> crate::mux::MuxRuntimeStatus {
    view(app, session).snapshot()
}
#[cfg(test)]
mod tests {
    use super::{build, unchanged};
    use crate::tui::app::{session_runtime::SessionView, state::App};

    #[test]
    fn owned_status_matches_its_borrowed_source() {
        let app = App::default();
        let session = SessionView::default();
        assert!(unchanged(&build(&app, &session), &app, &session));
    }
}
