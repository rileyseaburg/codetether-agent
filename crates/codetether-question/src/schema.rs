/// Return the provider-facing OpenCode-style batch question schema.
///
/// # Arguments
/// None.
/// # Returns
/// JSON Schema with labeled choices and optional multiple selection.
/// # Examples
/// ```
/// assert_eq!(codetether_question::parameters()["properties"]["questions"]["minItems"], 1);
/// ```
pub fn parameters() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "questions": {
            "type": "array", "minItems": 1,
            "description": "Questions to ask. Custom answers are offered automatically.",
            "items": {
                "type": "object",
                "properties": {
                    "question": {"type": "string", "minLength": 1},
                    "header": {"type": "string", "maxLength": 30},
                    "options": {"type": "array", "items": {
                        "type": "object", "properties": {
                            "label": {"type": "string", "minLength": 1},
                            "description": {"type": "string"}
                        }, "required": ["label", "description"]
                    }},
                    "multiple": {"type": "boolean", "default": false}
                }, "required": ["question", "header", "options"]
            }
        }}, "required": ["questions"]
    })
}
