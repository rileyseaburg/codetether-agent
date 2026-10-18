//! A final message cannot discard a correction or dispatch tools after cancel.

use super::{control_test_provider::Mock, driver, image_test_state};
use codetether_native_session_runtime::ControlHandle;
use codetether_tool_core::ToolResult;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn steering_during_final_response_continues_the_delegated_loop() {
    let control = ControlHandle::new();
    let provider = Arc::new(Mock {
        control: control.clone(),
        cancel: false,
        requests: Mutex::new(vec![]),
    });
    let mut state = control
        .scope(async { image_test_state::state(ToolResult::success("unused")) })
        .await;
    state.inner.max_steps = 3;
    state.inner.provider = provider.clone();
    driver::execute(&mut state).await.unwrap();
    let requests = provider.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert!(requests[1].messages.iter().any(|message| message.content.iter().any(|part| {
        matches!(part, codetether_provider_core::ContentPart::Text { text } if text == "Inspect cloned repos")
    })));
}

#[tokio::test]
async fn cancellation_during_completion_prevents_tool_dispatch() {
    let control = ControlHandle::new();
    let provider = Arc::new(Mock {
        control: control.clone(),
        cancel: true,
        requests: Mutex::new(vec![]),
    });
    let mut state = control
        .scope(async { image_test_state::state(ToolResult::success("unused")) })
        .await;
    state.inner.provider = provider;
    assert!(driver::execute(&mut state).await.is_err());
    assert_eq!(state.inner.conversation.tool_calls, 0);
    assert!(
        !state
            .inner
            .conversation
            .messages
            .iter()
            .any(|message| matches!(message.role, codetether_provider_core::Role::Tool))
    );
}
