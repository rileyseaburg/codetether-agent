use crate::tool::ToolResult;
use serde_json::json;
use std::path::Path;

pub(super) fn generated(data_url: String, path: Option<&Path>) -> ToolResult {
    let path_text = path.map(|value| value.display().to_string());
    let output = path_text.as_ref().map_or_else(
        || "Generated image returned; local artifact was not saved".into(),
        |path| format!("Generated image saved to {path}"),
    );
    let mut image = json!({"image_url": data_url.clone()});
    if let Some(path) = &path_text {
        image["output_hint"] = json!(format!("Generated image saved to {path}"));
    }
    let mut result = ToolResult::success(output)
        .with_metadata(
            "image_data_url",
            json!({
                "data_url": data_url,
                "mime_type": "image/png",
                "detail": "auto"
            }),
        )
        .with_metadata("generated_image", image);
    if let Some(path) = path_text {
        result = result.with_metadata("saved_path", json!(path));
    }
    result
}