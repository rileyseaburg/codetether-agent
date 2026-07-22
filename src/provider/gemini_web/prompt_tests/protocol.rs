use super::{ContentPart, Message, Role, prompt};

#[test]
fn surrounds_history_with_gemini_safety_rules() {
    let rendered = prompt::render(
        &[Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "go".into() }],
        }],
        &[],
    )
    .unwrap();

    assert!(rendered.starts_with("System: <gemini_web_tool_protocol>"));
    assert!(rendered.contains("Never invent, predict, or reuse a session ID"));
    assert!(rendered.contains("wrapper success does not mean"));
    assert!(
        rendered.ends_with("Do not predict tool results or emit dependent calls in one batch.")
    );
}

#[test]
fn corrective_retry_stays_under_web_prompt_limit() {
    let messages = [Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "x".repeat(400_000),
        }],
    }];
    let original = prompt::render(&messages, &[]).unwrap();
    let corrected = prompt::retry(&original, &"invalid".repeat(1_000));
    assert!(corrected.len() <= 256 * 1024);
}
