//! YouTube tool argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": super::schema_fields::action(),
            "podcast_id": {
                "type": "string",
                "description": "Podcast ID (required for publish_episode)"
            },
            "episode_id": {
                "type": "string",
                "description": "Episode ID (required for publish_episode)"
            },
            "privacy_status": super::schema_fields::privacy_status(),
            "custom_title": {
                "type": "string",
                "description": "Override the episode title for the YouTube video"
            },
            "custom_description": {
                "type": "string",
                "description": "Override the episode description for YouTube"
            },
            "tags": {
                "type": "string",
                "description": "Comma-separated tags for the YouTube video"
            },
            "file_path": {
                "type": "string",
                "description": "Path to video file (required for upload_video)"
            },
            "title": {
                "type": "string",
                "description": "Video title (required for upload_video)"
            },
            "description": {
                "type": "string",
                "description": "Video description (for upload_video)"
            },
            "category_id": {
                "type": "string",
                "description": "YouTube category ID (default: 28 = Science & Technology)",
                "default": "28"
            }
        },
        "required": ["action"]
    })
}
