//! Encode the original forage score-hook input.
use super::ScoreInput;
use serde_json::{Value, json};
pub(super) fn event(input: &ScoreInput<'_>) -> Value {
    json!({
        "base_score": input.base_score,
        "remaining": input.remaining,
        "moonshot_alignment": input.moonshot_alignment,
        "okr": { "id": input.okr.id, "title": input.okr.title },
        "key_result": { "id": input.kr.id, "title": input.kr.title }
    })
}
