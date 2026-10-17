use super::validate;
use serde_json::{Value, json};

#[test]
fn validates_nested_array_item_types() {
    let schema = json!({"type":"array","items":{"type":"object","properties":{
        "path":{"type":"string"}},"required":["path"]}});
    let error = validate(&json!([{"path": 7}]), &schema).unwrap_err();
    assert!(error.to_string().contains("item 0"));
}

#[test]
fn enforces_any_of_required_field_branches() {
    let schema = json!({"type":"object","anyOf":[
        {"required":["tool"]},{"required":["name"]}
    ]});
    assert!(validate(&json!({"tool":"read"}), &schema).is_ok());
    assert!(validate(&json!({"args":{}}), &schema).is_err());
}

#[test]
fn enforces_exactly_one_one_of_branch() {
    let schema = json!({"oneOf":[{"type":"string"},{"type":"object"}]});
    assert!(validate(&json!("task"), &schema).is_ok());
    assert!(validate(&json!(false), &schema).is_err());
}

#[test]
fn enforces_numeric_and_collection_bounds() {
    assert!(validate(&json!(0), &json!({"type":"integer","minimum":1})).is_err());
    assert!(validate(&json!([1, 2]), &json!({"type":"array","maxItems":1})).is_err());
}

#[test]
fn accepts_nullable_type_union() {
    let schema = json!({"type":["string","null"]});
    assert!(validate(&Value::Null, &schema).is_ok());
}
