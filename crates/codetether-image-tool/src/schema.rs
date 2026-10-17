//! schema for image loading.
use super::*;

pub(super) fn parameters() -> Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "Path to the image file (absolute or relative) or URL"
            },
            "detail": {
                "type": "string",
                "enum": ["low", "high", "auto"],
                "description": "Optional detail level for vision models (low=512x512, high=full resolution with 512x512 tiles)"
            }
        },
        "required": ["path"]
    })
}
