//! Registration of the status MCP tool and its argument schema.
use super::*;
pub(super) async fn register(server: &ServerCatalog, bridge: &Arc<BusBridge>) {
    let b = Arc::clone(bridge);
    server
        .register_tool(
            "bus_status",
            "Get the current status of the bus bridge: connection state, event count, \
             and buffer usage.",
            json!({
                "type": "object",
                "properties": {}
            }),
            super::status_handler::handler(b),
        )
        .await;
}
