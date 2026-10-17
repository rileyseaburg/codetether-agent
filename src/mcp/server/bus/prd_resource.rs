//! Registration and reading of the Ralph PRD resource.
use super::*;
pub(super) async fn register(server: &McpServer) {
    server
        .register_resource(
            "codetether://ralph/prd",
            "Ralph PRD",
            "Current PRD JSON with story pass/fail status",
            Some("application/json"),
            Arc::new(|_uri| {
                let text = std::fs::read_to_string("prd.json")
                    .unwrap_or_else(|_| r#"{"error": "prd.json not found"}"#.to_string());
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents {
                        uri: "codetether://ralph/prd".to_string(),
                        mime_type: Some("application/json".to_string()),
                        content: ResourceContent::Text { text },
                    }],
                })
            }),
        )
        .await;
}
