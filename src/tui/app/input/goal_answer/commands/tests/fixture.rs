//! Native session/runtime fixture; no provider or sub-agent is started.

use crate::tui::app::{
    session_runtime::{self, SessionSlot, TuiSessionHandle},
    state::App,
};

pub(super) async fn setup() -> (App, SessionSlot, TuiSessionHandle) {
    let session = crate::session::tasks::answer_review_test_support::session().await;
    let mut app = App::default();
    app.state.session_id = Some(session.id.clone());
    let (event_tx, _events) = tokio::sync::mpsc::channel(4);
    let (notice_tx, _notices) = tokio::sync::mpsc::channel(4);
    (
        app,
        SessionSlot::new(session),
        session_runtime::spawn(event_tx, notice_tx),
    )
}

pub(super) async fn submit(
    app: &mut App,
    slot: &mut SessionSlot,
    runtime: &TuiSessionHandle,
    text: &str,
) {
    app.state.input = text.into();
    crate::tui::app::input::chat_submit::handle_enter_chat(
        app,
        std::path::Path::new("."),
        slot,
        &None,
        &None,
        runtime,
    )
    .await;
}
