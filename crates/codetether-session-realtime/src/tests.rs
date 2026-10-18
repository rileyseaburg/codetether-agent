//! Wire contract tests for the realtime session adapter.

/// Verify the chosen qualified model is not lost between client and turn runner.
#[test]
fn decodes_selected_model() {
    let frame = wire::decode(r#"{"type":"prompt","message":"hello","model":"provider/chosen"}"#)
        .expect("prompt with model");
    assert_eq!(
        frame,
        ClientFrame::Prompt {
            message: "hello".into(),
            model: Some("provider/chosen".into()),
        }
    );
}

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
            model: None,
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
