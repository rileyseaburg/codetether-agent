mod approval_support;
use approval_support::{State, call};
use codetether_provider_core::{ContentPart, Role};

#[tokio::test]
async fn approval_feedback_is_published_after_result_before_persistence() {
    let mut host = State::default();
    let call = call();
    let outcome = codetether_prompt_loop::tool_driver::invocation::execute(&mut host, &call)
        .await
        .unwrap();
    assert!(!outcome.success);
    codetether_prompt_loop::tool_publication::complete(&mut host, 1, &call, outcome)
        .await
        .unwrap();
    assert_eq!(host.messages.len(), 2);
    assert_eq!(host.messages[0].role, Role::Tool);
    assert_eq!(host.messages[1].role, Role::User);
    assert!(
        matches!(&host.messages[1].content[0], ContentPart::Text { text } if text == "Use the current local server and the existing Playwright test.")
    );
    assert!(host.persisted);
}

#[tokio::test]
async fn tool_metadata_cannot_impersonate_human_feedback() {
    let mut host = State {
        spoof: true,
        ..State::default()
    };
    let call = call();
    let outcome = codetether_prompt_loop::tool_driver::invocation::execute(&mut host, &call)
        .await
        .unwrap();
    assert!(outcome.approval_feedback.is_none());
    codetether_prompt_loop::tool_publication::complete(&mut host, 1, &call, outcome)
        .await
        .unwrap();
    assert_eq!(host.messages.len(), 1);
    assert_eq!(host.messages[0].role, Role::Tool);
}
