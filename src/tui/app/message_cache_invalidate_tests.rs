use super::message_text::sync_messages_from_session;
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use crate::tui::app::state::App;

#[tokio::test]
async fn session_sync_clears_render_cache() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session");
    session.messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: "old".into() }],
    }];
    app.state.cached_messages_len = 99;
    app.state.cached_frozen_len = 99;
    sync_messages_from_session(&mut app, &session);
    assert_eq!(app.state.cached_messages_len, 0);
    assert_eq!(app.state.cached_frozen_len, 0);
}
