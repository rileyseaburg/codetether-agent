//! The modal owns answer input, leaving the chat draft untouched.
use super::*;
use codetether_question::Request;
use serde_json::json;

#[tokio::test]
async fn question_keys_and_paste_submit_custom_text() {
    let mut app = App::default();
    app.state.session_id = Some("question-keys".into());
    app.state.input = "existing chat".into();
    let request = Request::parse(json!({"question":"Choose?", "options":["A"]})).unwrap();
    let job = tokio::spawn(runtime::scope(
        "question-keys".into(),
        runtime::ask(request, None),
    ));
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while runtime::pending("question-keys").is_none() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(paste(&mut app, "custom"));
    assert!(handle(
        &mut app,
        KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE)
    ));
    assert!(handle(
        &mut app,
        KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)
    ));
    assert_eq!(job.await.unwrap().unwrap(), vec![vec!["custom!"]]);
    assert_eq!(app.state.input, "existing chat");
    assert!(!handle(
        &mut app,
        KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)
    ));
}
