use super::is_upgrade_required;

#[test]
fn forbidden_websocket_handshake_switches_to_http() {
    assert!(is_upgrade_required(&anyhow::anyhow!(
        "HTTP error: 403 Forbidden"
    )));
}
