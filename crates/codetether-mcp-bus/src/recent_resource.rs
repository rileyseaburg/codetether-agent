//! Registration and reading of the recent bus events resource.
use super::*;
pub(super) async fn register(server: &ServerCatalog, bridge: &Arc<BusBridge>) {
    let b = Arc::clone(bridge);
    server
        .register_resource(
            "codetether://bus/events/recent",
            "Recent Bus Events",
            "Last 100 events from the agent bus (JSON array of BusEnvelope)",
            Some("application/json"),
            Arc::new(move |_uri| {
                let events = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current()
                        .block_on(async { b.recent_events(None, 100, None).await })
                });
                let text =
                    serde_json::to_string_pretty(&events).unwrap_or_else(|_| "[]".to_string());
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents {
                        uri: "codetether://bus/events/recent".to_string(),
                        mime_type: Some("application/json".to_string()),
                        content: ResourceContent::Text { text },
                    }],
                })
            }),
        )
        .await;
}
