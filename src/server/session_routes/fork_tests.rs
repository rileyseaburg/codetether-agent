//! Non-destructive edit branching and stale-target rejection.
use super::service;
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

#[test]
fn edit_fork_requires_execute_authority() {
    assert_eq!(
        crate::server::match_policy_rule("/api/session/test/fork", "POST"),
        Some("agent:execute")
    );
}

fn message(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

#[tokio::test]
async fn editing_preserves_original_and_forks_only_prior_context() {
    let mut source = Session::new().await.unwrap();
    source.add_message(message(Role::User, "First"));
    source.add_message(message(Role::Assistant, "First answer"));
    source.add_message(message(Role::User, "Change this"));
    source.add_message(message(Role::Assistant, "Outdated answer"));
    source.metadata.model = Some("provider/chosen".into());
    source.metadata.shared = true;
    let branch = service::create(&source, 2, "Change this").await.unwrap();
    assert_ne!(branch.id, source.id);
    assert_eq!(source.messages.len(), 4);
    assert_eq!(branch.messages.len(), 2);
    assert_eq!(branch.metadata.model, source.metadata.model);
    assert!(!branch.metadata.shared);
    assert!(branch.metadata.run_checkpoint.is_none());
}

#[tokio::test]
async fn stale_or_non_user_targets_are_rejected() {
    let mut source = Session::new().await.unwrap();
    source.add_message(message(Role::User, "Original"));
    source.add_message(message(Role::Assistant, "Answer"));
    assert!(service::create(&source, 0, "Different").await.is_err());
    assert!(service::create(&source, 1, "Answer").await.is_err());
    assert!(service::create(&source, 9, "Missing").await.is_err());
    assert_eq!(source.messages.len(), 2);
}
