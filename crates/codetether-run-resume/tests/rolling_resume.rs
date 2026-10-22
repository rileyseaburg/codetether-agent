mod support;
use support::Session;

#[tokio::test]
async fn plain_resumes_even_when_window_length_never_grows() {
    let mut session = Session::new("rolling-plain");
    let start = session.full.len();
    codetether_run_resume::execute_prompt_with_resume(
        &mut session,
        "repair",
        Some(1),
        Some(2),
        std::path::Path::new("."),
    )
    .await
    .unwrap();
    verify(session, start).await;
}
#[tokio::test]
async fn event_resumes_even_when_window_length_never_grows() {
    let mut session = Session::new("rolling-events");
    let start = session.full.len();
    let (tx, _rx) = tokio::sync::mpsc::channel(8);
    codetether_run_resume::execute_prompt_with_resume_events(
        &mut session,
        "repair",
        Some(1),
        Some(2),
        std::path::Path::new("."),
        tx,
    )
    .await
    .unwrap();
    verify(session, start).await;
}
async fn verify(mut session: Session, start: usize) {
    assert_eq!(session.attempts.len(), 3);
    assert_eq!(session.window.len(), 2);
    assert_eq!(session.full.len(), start + 3);
    let checkpoint = session.checkpoint.take().unwrap();
    assert_eq!(checkpoint.message_count, session.full.len());
    assert!(checkpoint.user_context[0].text.contains("Do not deploy"));
    let cold = codetether_run_checkpoint::storage::load(&session)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(cold, checkpoint);
    for prompt in &session.attempts[1..] {
        assert!(prompt.contains("Do not deploy"));
    }
}

#[tokio::test]
async fn resume_reconciles_same_length_corrections_with_canonical_history() {
    use codetether_run_resume::RunSession;
    let mut session = Session::new("edited-checkpoint");
    session.checkpoint = Some(
        session
            .build_checkpoint("repair", 2, std::path::Path::new("."))
            .await
            .unwrap(),
    );
    session.full[0].content = vec![codetether_provider_core::ContentPart::Text {
        text: "New correction: no network access".into(),
    }];
    let plan = codetether_run_resume::resume_plan(&session, 1)
        .await
        .unwrap()
        .unwrap();
    assert!(plan.prompt.contains("New correction: no network access"));
    session.checkpoint.as_mut().unwrap().session_id = "another-session".into();
    assert!(
        codetether_run_resume::resume_plan(&session, 1)
            .await
            .is_err()
    );
}
