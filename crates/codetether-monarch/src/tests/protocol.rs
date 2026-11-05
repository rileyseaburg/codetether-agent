//! Response identity and error-redaction contracts for local fixtures.
use crate::response;
use serde_json::json;

#[test]
fn response_identity_errors_and_events() {
    assert_eq!(response::event(": keepalive\n\n", 1).unwrap(), None);
    assert_eq!(
        response::event(
            "data: {\"jsonrpc\":\"2.0\",\n data: ignored\ndata: \"id\":1,\"result\":{}}\n\n",
            1
        )
        .unwrap(),
        Some(json!({}))
    );
    assert!(response::result(json!({"jsonrpc":"2.0","id":2,"result":{}}), 1).is_err());
    let error = response::result(
        json!({"jsonrpc":"2.0","id":1,"error":{"message":"private data"}}),
        1,
    )
    .unwrap_err();
    assert!(!error.to_string().contains("private data"));
}
