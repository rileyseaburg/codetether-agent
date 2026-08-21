//! Wire contract tests for the realtime session adapter.

use super::frames::ClientFrame;
use super::wire;

/// Verify a prompt frame retains user text without reinterpretation.
#[test]
fn decodes_prompt_frame() {
    let frame = wire::decode(r#"{"type":"prompt","message":"inspect now"}"#).expect("prompt frame");
    assert_eq!(
        frame,
        ClientFrame::Prompt {
            message: "inspect now".into(),
        }
    );
}

/// Verify steering frames carry a correlation id for acknowledgements.
#[test]
fn decodes_steering_frame() {
    let frame = wire::decode(r#"{"type":"steer","request_id":"s1","message":"adjust"}"#)
        .expect("steering frame");
    assert_eq!(
        frame,
        ClientFrame::Steer {
            request_id: "s1".into(),
            message: "adjust".into(),
        }
    );
}
