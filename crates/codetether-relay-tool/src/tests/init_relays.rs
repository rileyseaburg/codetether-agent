//! Initialize the two isolated relays used by the scope regression.
use super::*;
pub(super) async fn init_relays(tool: &RelayAutoChatTool, relay_a: String, relay_b: String) {
    tool.init_relay(
        Some(relay_a.clone()),
        Some("task a".to_string()),
        None,
        None,
    )
    .await
    .expect("init relay a");
    tool.init_relay(
        Some(relay_b.clone()),
        Some("task b".to_string()),
        None,
        None,
    )
    .await
    .expect("init relay b");
}
