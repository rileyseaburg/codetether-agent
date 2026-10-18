//! Registration of the events MCP tool and its argument schema.
use super::*;
pub(super) async fn register(server: &ServerCatalog, bridge: &Arc<BusBridge>) {
    let b = Arc::clone(bridge);
    server.register_tool(
            "bus_events",
            "Query recent events from the agent bus. Returns BusEnvelope JSON objects \
             matching the optional topic filter (supports wildcards like 'ralph.*').",
            json!({
                "type": "object",
                "properties": {
                    "topic_filter": {
                        "type": "string",
                        "description": "Topic pattern to filter (e.g. 'ralph.*', 'agent.*', '*'). Default: all."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Max events to return (default: 50, max: 500)"
                    }
                }
            }),
            super::events_handler::handler(b),
        )
        .await;
}
