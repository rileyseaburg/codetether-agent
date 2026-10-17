//! Complete avatar argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": super::schema_fields_0::action(),
            "text": super::schema_fields_0::text(),
            "audio_url": super::schema_fields_0::audio_url(),
            "audio_file": super::schema_fields_0::audio_file(),
            "model_video": super::schema_fields_1::model_video(),
            "voice_id": super::schema_fields_1::voice_id(),
            "podcast_id": super::schema_fields_1::podcast_id(),
            "episode_id": super::schema_fields_1::episode_id(),
            "upload_youtube": super::schema_fields_2::upload_youtube(),
            "privacy_status": super::schema_fields_2::privacy_status(),
            "name": super::schema_fields_2::name(),
            "file_path": super::schema_fields_2::file_path()
        },
        "required": ["action"]
    })
}
