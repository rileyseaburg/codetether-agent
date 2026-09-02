use super::upsert_active_session;
use crate::session::Session;
use crate::tui::app::state::App;

#[tokio::test]
async fn finished_session_moves_to_top_without_duplicates() {
    let mut app = App::default();
    let mut session = Session::new().await.unwrap();
    session.title = Some("first".into());
    upsert_active_session(&mut app, &session);
    let other = Session::new().await.unwrap();
    upsert_active_session(&mut app, &other);
    session.title = Some("renamed".into());
    upsert_active_session(&mut app, &session);

    assert_eq!(app.state.sessions.len(), 2);
    assert_eq!(app.state.sessions[0].id, session.id);
    assert_eq!(app.state.sessions[0].title.as_deref(), Some("renamed"));
    assert_eq!(app.state.sessions[1].id, other.id);
    assert_eq!(app.state.selected_session, 0);
}
