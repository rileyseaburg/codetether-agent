//! Complete podcast argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": super::schema_fields_0::action(),
            "podcast_id": super::schema_fields_0::podcast_id(),
            "episode_id": super::schema_fields_0::episode_id(),
            "title": super::schema_fields_0::title(),
            "description": super::schema_fields_1::description(),
            "author": super::schema_fields_1::author(),
            "email": super::schema_fields_1::email(),
            "category": super::schema_fields_1::category(),
            "subcategory": super::schema_fields_2::subcategory(),
            "language": super::schema_fields_2::language(),
            "script": super::schema_fields_2::script(),
            "voice_id": super::schema_fields_2::voice_id(),
            "episode_number": super::schema_fields_3::episode_number(),
            "season_number": super::schema_fields_3::season_number()
        },
        "required": ["action"]
    })
}
