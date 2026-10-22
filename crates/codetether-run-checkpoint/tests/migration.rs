use codetether_provider_core::{ContentPart, Message, Role};
use codetether_run_checkpoint::{RunCheckpoint, auto_resume_prompt};

#[test]
fn legacy_checkpoints_deserialize_without_claiming_verified_actions() {
    let cp = RunCheckpoint::from_session_messages("fix", 10, "s", None, 0, &[]);
    let mut value = serde_json::to_value(cp).unwrap();
    let object = value.as_object_mut().unwrap();
    for key in [
        "evidence_version",
        "actions",
        "user_context",
        "assistant_context",
    ] {
        object.remove(key);
    }
    object.insert("completed_actions".into(), serde_json::json!(["edit"]));
    let legacy: RunCheckpoint = serde_json::from_value(value).unwrap();
    assert_eq!(legacy.evidence_version, 0);
    assert!(auto_resume_prompt(&legacy).contains("Legacy action notes (unverified"));
}

#[test]
fn runtime_reminders_do_not_become_human_corrections() {
    let messages = [Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "Continue working toward the active thread goal.\n<objective>fix</objective>"
                .into(),
        }],
    }];
    let cp = RunCheckpoint::from_session_messages("fix", 10, "s", None, 1, &messages);
    assert!(cp.user_context.is_empty());
}
