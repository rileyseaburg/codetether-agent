use codetether_provider_core::{ContentPart, Message, Role};
use codetether_run_checkpoint::{ActionStatus, RunCheckpoint, auto_resume_prompt};

fn message(role: Role, part: ContentPart) -> Message {
    Message {
        role,
        content: vec![part],
    }
}
fn call(id: &str) -> Message {
    message(
        Role::Assistant,
        ContentPart::ToolCall {
            id: id.into(),
            name: "edit".into(),
            arguments: "{}".into(),
            thought_signature: None,
        },
    )
}
fn reply(id: &str, content: &str) -> Message {
    message(
        Role::Tool,
        ContentPart::ToolResult {
            tool_call_id: id.into(),
            content: content.into(),
        },
    )
}

#[test]
fn only_explicit_success_is_completed_and_calls_are_matched_by_id() {
    let messages = vec![
        call("ok"),
        call("failed"),
        call("pending"),
        call("unknown"),
        reply(
            "failed",
            "Tool call feedback\n- tool: edit\n- status: failure\n\nOutput:\npermission denied",
        ),
        reply(
            "ok",
            "Tool call feedback\n- tool: edit\n- status: success\n\nOutput:\nerror handler updated",
        ),
        reply("unknown", "some result"),
        reply("orphan", "Error: orphan failure"),
    ];
    let cp = RunCheckpoint::from_session_messages("fix", 10, "s", None, 108, &messages);
    assert_eq!(cp.completed_actions, ["edit"]);
    assert_eq!(
        cp.actions.iter().map(|a| a.status).collect::<Vec<_>>(),
        [
            ActionStatus::Success,
            ActionStatus::Failed,
            ActionStatus::Attempted,
            ActionStatus::Returned
        ]
    );
    assert_eq!(cp.actions[0].source_turn, 100);
    assert_eq!(cp.actions[0].result_turn, Some(105));
    assert_eq!(cp.blockers.len(), 2);
}

#[test]
fn complete_user_corrections_survive_and_override_the_inferred_next_action() {
    let correction = format!("Keep investigating\n{}\nDo not deploy.", "🦀".repeat(600));
    let messages = vec![
        message(
            Role::Assistant,
            ContentPart::Text {
                text: "Deploy now".into(),
            },
        ),
        message(
            Role::User,
            ContentPart::Text {
                text: correction.clone(),
            },
        ),
    ];
    let cp = RunCheckpoint::from_session_messages("investigate", 10, "s", None, 32, &messages);
    assert_eq!(cp.user_context[0].text, correction);
    assert_eq!(cp.user_context[0].source_turn, 31);
    assert!(cp.next_intended_action.contains("turn 31"));
    let prompt = auto_resume_prompt(&cp);
    assert!(prompt.contains(&correction));
    assert!(prompt.contains("unverified"));
}

#[test]
fn unicode_assistant_text_does_not_panic() {
    let messages = [message(
        Role::Assistant,
        ContentPart::Text {
            text: "🦀".repeat(501),
        },
    )];
    let cp = RunCheckpoint::from_session_messages("fix", 10, "s", None, 1, &messages);
    assert_eq!(cp.next_intended_action.chars().count(), 500);
}
