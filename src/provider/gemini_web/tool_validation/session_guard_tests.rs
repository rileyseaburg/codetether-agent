use super::validate;
use crate::provider::{ContentPart, Message, Role};

fn result(text: &str) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "call-1".into(),
            content: text.into(),
        }],
    }
}

fn poll(id: u64) -> (String, String) {
    ("write_stdin".into(), format!(r#"{{"session_id":{id}}}"#))
}

#[test]
fn accepts_id_from_trailing_real_tool_result() {
    let messages = vec![result("Script running with session ID 17")];
    assert!(validate(&[poll(17)], &messages).is_ok());
}

#[test]
fn rejects_invented_or_stale_id() {
    let messages = vec![result("Script running with session ID 17")];
    let error = validate(&[poll(4)], &messages).unwrap_err();
    assert!(error.to_string().contains("not present"));
}

#[test]
fn rejects_poll_batched_with_command_that_creates_its_session() {
    let calls = vec![
        ("exec_command".into(), r#"{"cmd":"sleep 1"}"#.into()),
        poll(17),
    ];
    let error = validate(&calls, &[]).unwrap_err();
    assert!(error.to_string().contains("cannot share a batch"));
}
