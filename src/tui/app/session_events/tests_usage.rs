use super::super::*;
use crate::session::{Session, SessionEvent};

#[tokio::test]
async fn usage_report_updates_latency_snapshot() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");
    handle_session_event(
        &mut app,
        &mut session,
        &None,
        SessionEvent::UsageReport {
            model: "openai/gpt-5.4".to_string(),
            prompt_tokens: 120,
            completion_tokens: 64,
            duration_ms: 1_250,
        },
    )
    .await;
    assert_eq!(
        app.state.last_completion_model.as_deref(),
        Some("openai/gpt-5.4")
    );
    assert_eq!(app.state.last_completion_latency_ms, Some(1_250));
    assert_eq!(app.state.last_completion_prompt_tokens, Some(120));
    assert_eq!(app.state.last_completion_output_tokens, Some(64));
}
