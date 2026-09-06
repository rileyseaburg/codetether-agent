//! In-process MCP boundary: an actual image tool returns image blocks, not prose.

use crate::mcp::McpServer;
use crate::tool::{ToolRegistry, image::ImageTool};
use base64::Engine;
use serde_json::json;
use std::sync::Arc;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mcp_registered_image_tool_preserves_pixels_on_wire() {
    let mut encoded = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        1,
        1,
        image::Rgba([200, 100, 50, 255]),
    ))
    .write_to(&mut encoded, image::ImageFormat::Png)
    .unwrap();
    let bytes = encoded.into_inner();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("pixel.png");
    std::fs::write(&path, &bytes).unwrap();
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(ImageTool::new()));
    let server = McpServer::new_local().with_tool_registry(Arc::new(registry));
    server.setup_tools_public().await;
    let result = server
        .call_tool_direct("image", json!({"path": path}))
        .await
        .unwrap();
    assert!(!result.is_error);
    let wire = serde_json::to_value(result).unwrap();
    let image = wire["content"]
        .as_array()
        .unwrap()
        .iter()
        .find(|part| part["type"] == "image")
        .expect("MCP image block");
    assert_eq!(image["mimeType"], "image/png");
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(image["data"].as_str().unwrap())
            .unwrap(),
        bytes
    );
    assert!(
        !wire["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("base64")
    );
}
