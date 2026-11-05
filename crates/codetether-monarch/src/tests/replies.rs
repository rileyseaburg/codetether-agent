//! Expected responses and assertions for each local MCP lifecycle step.
use serde_json::{Value, json};

pub(super) fn for_step(
    step: usize,
    headers: &str,
    request: &Value,
) -> (&'static str, &'static str, String) {
    match step {
        0 => {
            assert_eq!(request["method"], "initialize");
            ("200 OK", "application/json", json!({"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-03-26","capabilities":{"tools":{}}}}).to_string())
        }
        1 => {
            assert_eq!(request["method"], "notifications/initialized");
            ("202 Accepted", "application/json", String::new())
        }
        2 => {
            assert_eq!(request["method"], "tools/list");
            ("200 OK", "text/event-stream", "event: message\r\ndata: {\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{\"tools\":[]}}\r\n\r\n".into())
        }
        3 => {
            assert!(headers.starts_with("delete /mcp"));
            ("204 No Content", "application/json", String::new())
        }
        _ => unreachable!(),
    }
}
