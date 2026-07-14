use serde_json::{Value, json};

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "properties": {
            "prompt": {"type": "string", "description": "Detailed rewritten image prompt"},
            "referenced_image_paths": {
                "type": "array", "maxItems": 5,
                "items": {"type": "string"},
                "description": "Local images to edit; omit when generating a new image"
            },
            "num_last_images_to_include": {
                "type": "integer", "minimum": 1, "maximum": 5,
                "description": "Number of recent image_gen results to edit"
            }
        },
        "required": ["prompt"]
    })
}
