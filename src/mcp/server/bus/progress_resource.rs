//! Registration and reading of the Ralph progress resource.
use super::*;
pub(super) async fn register(server: &McpServer) {
    server
        .register_resource(
            "codetether://ralph/progress",
            "Ralph Progress",
            "progress.txt content from the current Ralph run",
            Some("text/plain"),
            Arc::new(|_uri| {
                let text = std::fs::read_to_string("progress.txt")
                    .unwrap_or_else(|_| "(no progress.txt found)".to_string());
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents {
                        uri: "codetether://ralph/progress".to_string(),
                        mime_type: Some("text/plain".to_string()),
                        content: ResourceContent::Text { text },
                    }],
                })
            }),
        )
        .await;
}
