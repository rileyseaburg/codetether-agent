use super::session_fork::fork_if_truncated;
use super::session_loader::load_session_for_tui;
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

#[tokio::test]
async fn loads_native_session_with_tail_window() {
    let temp = tempfile::tempdir().unwrap();
    unsafe {
        std::env::set_var("CODETETHER_DATA_DIR", temp.path());
        std::env::set_var("CODETETHER_SESSION_RESUME_WINDOW", "2");
    }
    let mut session = Session::new().await.unwrap();
    session.id = "tail-native-session".to_string();
    for text in ["one", "two", "three"] {
        session.messages.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: text.into() }],
        });
    }
    session.save().await.unwrap();
    let mut loaded = load_session_for_tui("tail-native-session").await.unwrap();
    let original = fork_if_truncated(&mut loaded.session, loaded.dropped);
    assert_eq!(loaded.session.messages.len(), 2);
    assert_eq!(loaded.dropped, 1);
    assert_eq!(original.as_deref(), Some("tail-native-session"));
    assert_ne!(loaded.session.id, "tail-native-session");
}
