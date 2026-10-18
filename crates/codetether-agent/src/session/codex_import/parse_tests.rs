//! Parsing preserves the complete transcript, including developer messages.
use super::{parse_codex_session_from_path, write_codex_fixture};
use crate::provider::{ContentPart, Role};
use tempfile::tempdir;

#[test]
fn parses_codex_session_into_native_session() {
    let temp = tempdir().expect("tempdir");
    let workspace = temp.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("workspace");
    let path = write_codex_fixture(temp.path(), &workspace);
    let session = parse_codex_session_from_path(&path, Some("Imported from Codex")).expect("parse");
    assert_eq!(session.id, "019d2acd-8b3f-70e0-b019-854d52272660");
    assert_eq!(session.metadata.model.as_deref(), Some("gpt-5.4"));
    assert_eq!(session.usage.total_tokens, 15);
    let roles: Vec<_> = session
        .messages
        .iter()
        .map(|message| &message.role)
        .collect();
    assert_eq!(
        roles,
        vec![
            &Role::Developer,
            &Role::User,
            &Role::Assistant,
            &Role::Assistant,
            &Role::Tool,
            &Role::Assistant,
        ]
    );
    assert!(matches!(
        &session.messages[0].content[0],
        ContentPart::Text { text } if text == "skip"
    ));
    assert!(matches!(
        session.messages[2].content[0],
        ContentPart::Thinking { .. }
    ));
    assert!(matches!(
        &session.messages[3].content[0],
        ContentPart::ToolCall { id, .. } if id == "call_1"
    ));
    assert!(matches!(
        &session.messages[4].content[0],
        ContentPart::ToolResult { tool_call_id, .. } if tool_call_id == "call_1"
    ));
}
