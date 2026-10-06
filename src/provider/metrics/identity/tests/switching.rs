//! Freshness, idempotence and caller-owned lookalike coverage.
use super::{inject, message, request, text};
use crate::provider::Role;

#[test]
fn harness_identity_replaces_stale_context_on_model_and_provider_switch() {
    let mut first = inject(request("model-a"), "provider-a", "model-a");
    first.model = "model-b".into();
    let second = inject(first, "provider-b", "model-b");
    let third = inject(second.clone(), "provider-b", "model-b");
    assert_eq!(
        serde_json::to_value(&third.messages).unwrap(),
        serde_json::to_value(&second.messages).unwrap()
    );
    assert_eq!(third.messages.len(), 3);
    assert!(text(&third.messages[0]).contains(r#""model":"model-b""#));
    assert!(text(&third.messages[0]).contains(r#""provider":"provider-b""#));
    assert!(!text(&third.messages[0]).contains("model-a"));
    assert!(!text(&third.messages[0]).contains("provider-a"));
}

#[test]
fn harness_identity_preserves_user_and_mixed_system_lookalikes() {
    let marker = format!(
        "{}not harness metadata{}",
        super::super::START,
        super::super::END
    );
    let mut original = request("model-a");
    original.messages.push(message(Role::User, &marker));
    original.messages.push(message(
        Role::System,
        &format!("Keep this prompt\n{marker}"),
    ));
    let injected = inject(original.clone(), "provider-a", "model-a");
    assert_eq!(injected.messages.len(), original.messages.len() + 1);
    assert_eq!(
        serde_json::to_value(&injected.messages[1..]).unwrap(),
        serde_json::to_value(&original.messages).unwrap()
    );
}

#[test]
fn harness_identity_escapes_model_and_provider_as_json() {
    let injected = inject(request("model\"\nname"), "provider\"name", "model\"\nname");
    assert!(text(&injected.messages[0]).contains(r#""model":"model\"\nname""#));
    assert!(text(&injected.messages[0]).contains(r#""provider":"provider\"name""#));
}
