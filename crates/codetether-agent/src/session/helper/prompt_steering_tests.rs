//! End-to-end prompt-loop steering tests.

use super::prompt_events::run_prompt_with_events;
use super::prompt_steering_test_provider::SteeringProvider;
use crate::provider::ProviderRegistry;
use crate::session::Session;
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::test]
async fn final_response_steering_continues_the_active_run() {
    let mut session = Session::new().await.expect("session");
    session.agent = "chat".into();
    session.metadata.model = Some("mock/test".into());
    let provider = Arc::new(SteeringProvider::new(session.id.clone()));
    let mut registry = ProviderRegistry::new();
    registry.register(provider.clone());
    let (tx, mut rx) = mpsc::channel(64);
    tokio::spawn(async move { while rx.recv().await.is_some() {} });

    let result = run_prompt_with_events(
        &mut session,
        "hello",
        Vec::new(),
        tx,
        Arc::new(registry),
    )
    .await;

    assert!(result.is_ok());
    assert_eq!(provider.outcome(), (2, true));
}
