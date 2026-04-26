//! Tests for streaming prompt event behavior.

use super::prompt_events::run_prompt_with_events;
use super::prompt_events_test_provider::StreamContextErrorProvider;
use crate::provider::{ContentPart, Message, ProviderRegistry, Role};
use crate::session::Session;
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::test]
async fn streaming_context_errors_retry_with_forced_compaction() {
    let provider = Arc::new(StreamContextErrorProvider::new());
    let mut registry = ProviderRegistry::new();
    registry.register(provider.clone());
    let registry = Arc::new(registry);
    let (tx, _rx) = mpsc::channel(16);
    let mut session = Session::new().await.expect("session");
    session.metadata.model = Some("mock/test".into());
    for _ in 0..7 {
        session.add_message(text_message());
    }

    let result = run_prompt_with_events(&mut session, "hello", Vec::new(), tx, registry).await;

    assert!(result.is_ok());
    assert_eq!(provider.calls(), 2);
}

fn text_message() -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: "old".into() }],
    }
}
