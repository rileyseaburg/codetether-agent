use crate::tool::ToolResult;
use serde_json::json;
use std::path::Path;

pub(super) fn generated(data_url: String, path: &Path) -> ToolResult {
    let path_text = path.display().to_string();
    ToolResult::success(format!("Generated image saved to {path_text}"))
        .with_metadata(
            "image_data_url",
            json!({
                "data_url": data_url,
                "mime_type": "image/png",
                "detail": "auto"
            }),
        )
        .with_metadata(
            "generated_image",
            json!({
                "image_url": data_url,
                "output_hint": format!("Generated image saved to {path_text}")
            }),
        )
        .with_metadata("saved_path", json!(path_text))
}
