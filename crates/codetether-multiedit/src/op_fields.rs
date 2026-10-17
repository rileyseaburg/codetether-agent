//! Read optional snippets and determine whether Morph is requested.
use super::op::EditOp;
use serde_json::Value;
pub(super) fn parse(entry: &Value, file: String, morph_enabled: bool) -> EditOp {
    let old_string = entry
        .get("old_string")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let new_string = entry
        .get("new_string")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let instruction = entry
        .get("instruction")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let update = entry
        .get("update")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let use_morph = morph_enabled && (instruction.is_some() || update.is_some());

    EditOp {
        file,
        old_string,
        new_string,
        instruction,
        update,
        use_morph,
    }
}
