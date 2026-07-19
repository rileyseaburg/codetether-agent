use super::{capture, replay};
use crate::provider::openai_codex::turn_state::TurnStateStore;
use serde_json::json;
use tokio_tungstenite::tungstenite::http::Request;

#[test]
fn captures_first_array_header_value() {
    let states = TurnStateStore::default();
    capture(
        &states,
        "turn",
        &json!({
            "type": "response.metadata",
            "headers": {"X-Codex-Turn-State": ["sticky", "ignored"]}
        }),
    );
    assert_eq!(
        states.current("turn").get().map(String::as_str),
        Some("sticky")
    );
}

#[test]
fn replays_state_on_websocket_handshake() {
    let states = TurnStateStore::default();
    states.capture("turn", "sticky");
    let mut request = Request::new(());
    replay(&states, "turn", &mut request);
    assert_eq!(request.headers()["x-codex-turn-state"], "sticky");
}
