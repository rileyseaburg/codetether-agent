//! Injection preserves caller-owned conversation content and completion options.
use super::{Role, inject, request, text};
use crate::provider::ToolDefinition;

#[test]
fn harness_identity_preserves_prompt_and_request_options() {
    let mut original = request("model-a");
    original.tools.push(ToolDefinition {
        name: "inspect".into(),
        description: "Inspect a resource".into(),
        parameters: serde_json::json!({
            "type": "object", "properties": {"path": {"type": "string"}},
            "required": ["path"]
        }),
    });
    let injected = inject(original.clone(), "provider-a", "model-a");
    assert_eq!(injected.messages.len(), original.messages.len() + 1);
    assert!(matches!(injected.messages[0].role, Role::System));
    for (before, after) in original.messages.iter().zip(&injected.messages[1..]) {
        assert_eq!(
            serde_json::to_value(before).unwrap(),
            serde_json::to_value(after).unwrap()
        );
    }
    assert!(text(&injected.messages[0]).contains(r#""provider":"provider-a""#));
    assert!(text(&injected.messages[0]).contains(r#""model":"model-a""#));
    assert_eq!(injected.model, original.model);
    assert_eq!(
        (injected.temperature, injected.top_p, injected.max_tokens),
        (original.temperature, original.top_p, original.max_tokens)
    );
    assert_eq!(injected.stop, original.stop);
    assert_eq!(
        serde_json::to_value(&injected.tools).unwrap(),
        serde_json::to_value(&original.tools).unwrap()
    );
}
