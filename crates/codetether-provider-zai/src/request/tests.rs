//! Expectations for the two buffered request formats.
use super::{BufferedFormat, encode_buffered};
use serde_json::json;

#[test]
fn formats_preserve_their_existing_optional_fields() {
    let mut request = crate::tests::request("Explain the tradeoffs.", 1024);
    request.top_p = Some(0.9);
    request.stop = vec!["STOP".into()];
    let app = encode_buffered(
        &request,
        vec![],
        vec![],
        json!({"type":"enabled"}),
        BufferedFormat::Application,
    );
    let injected = encode_buffered(
        &request,
        vec![],
        vec![],
        json!({"type":"enabled"}),
        BufferedFormat::ExplicitControls,
    );
    for key in ["stream", "top_p", "stop"] {
        assert!(app.get(key).is_none(), "application must omit {key}");
    }
    assert_eq!(injected["stream"], false);
    assert_eq!(injected["top_p"], json!(0.9));
    assert_eq!(injected["stop"], json!(["STOP"]));
    for key in ["model", "messages", "temperature", "thinking", "max_tokens"] {
        assert_eq!(app[key], injected[key], "common field {key}");
    }
    assert!(app.get("tools").is_none());
    assert_eq!(app["max_tokens"], 1024);
}
