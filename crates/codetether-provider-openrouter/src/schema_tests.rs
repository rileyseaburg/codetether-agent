use super::sanitize;
use serde_json::json;

#[test]
fn removes_regex_constraints_recursively() {
    let schema = json!({
        "type": "object",
        "properties": {
            "id": {"type": "string", "pattern": "^[a-z]+$"},
            "nested": {
                "type": "array",
                "items": {"type": "string", "pattern": "x.*"}
            }
        }
    });
    let clean = sanitize(schema);
    assert!(clean.pointer("/properties/id/pattern").is_none());
    assert!(clean.pointer("/properties/nested/items/pattern").is_none());
}

#[test]
fn drops_required_from_non_object_any_of_branches() {
    let schema = json!({
        "type": "object",
        "properties": {
            "calls": {"anyOf": [
                {"type": "array", "required": ["name"]},
                {"type": "object", "properties": {"name": {"type": "string"}},
                 "required": ["name", "missing"]}
            ]}
        }
    });
    let clean = sanitize(schema);
    assert!(
        clean
            .pointer("/properties/calls/anyOf/0/required")
            .is_none()
    );
    assert_eq!(
        clean.pointer("/properties/calls/anyOf/1/required"),
        Some(&json!(["name"]))
    );
}
