use codetether_provider_core::{ContentPart, Message, Role};
use codetether_run_checkpoint::{
    CheckpointBuilder, RunCheckpoint, UserEvidence, auto_resume_prompt,
};
fn text(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
#[test]
fn streamed_history_matches_full_scan_and_rejects_a_missing_page() {
    let messages = vec![
        text(Role::User, "Do not deploy"),
        text(Role::Assistant, "Investigate failure"),
    ];
    let mut builder = CheckpointBuilder::default();
    assert!(builder.push(1, &messages[1]).is_err());
    for (index, message) in messages.iter().enumerate() {
        builder.push(index, message).unwrap();
    }
    let streamed = builder.finish("task", 2, "s", None);
    let scanned =
        RunCheckpoint::from_session_messages("task", 2, "s", None, messages.len(), &messages);
    assert_eq!(streamed.user_context, scanned.user_context);
    assert_eq!(streamed.assistant_context, scanned.assistant_context);
    assert_eq!(streamed.next_intended_action, scanned.next_intended_action);
    assert_eq!(streamed.message_count, 2);
}
#[test]
fn huge_checkpoint_retains_full_evidence_but_has_bounded_resume_prompt() {
    let mut cp = RunCheckpoint::from_session_messages("🦀".repeat(100_000), 2, "s", None, 0, &[]);
    cp.user_context = (0..500)
        .map(|source_turn| UserEvidence {
            source_turn,
            text: format!(
                "source {source_turn}\n{}\nDo not deploy.",
                "🦀".repeat(10_000)
            ),
        })
        .collect();
    cp.assistant_context = Some("unverified ".repeat(100_000));
    cp.blockers = vec!["x".repeat(50_000); 1_000];
    cp.completed_actions = vec!["x".repeat(50_000); 1_000];
    let prompt = auto_resume_prompt(&cp);
    assert!(prompt.len() <= 32_768, "{} bytes", prompt.len());
    assert!(prompt.contains("Do not deploy."));
    assert!(prompt.contains("context_browse"));
    assert!(prompt.contains("user turn 499"));
    assert_eq!(cp.user_context.len(), 500);
    assert!(cp.user_context[250].text.len() > 40_000);
    let restored: RunCheckpoint =
        serde_json::from_slice(&serde_json::to_vec(&cp).unwrap()).unwrap();
    assert_eq!(restored.user_context, cp.user_context);
}
