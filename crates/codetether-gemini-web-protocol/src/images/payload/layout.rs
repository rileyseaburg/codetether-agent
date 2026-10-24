use serde_json::{Value, json};
use uuid::Uuid;

pub(super) fn new(prompt: &str) -> Vec<Value> {
    let mut inner = vec![Value::Null; 99];
    inner[0] = json!([prompt, 0, null, null, null, null, 0]);
    inner[1] = json!(["en"]);
    inner[6] = json!([0]);
    inner[17] = json!([[1]]);
    inner[30] = json!([4, 16, 19]);
    for (slot, value) in [
        (7, 1),
        (10, 1),
        (11, 0),
        (18, 0),
        (27, 1),
        (49, 14),
        (53, 0),
        (67, 0),
        (68, 1),
        (79, 5),
        (91, 1),
        (96, 0),
        (98, 1),
    ] {
        inner[slot] = json!(value);
    }
    for slot in [54, 55, 61] {
        inner[slot] = json!([]);
    }
    inner[59] = json!(Uuid::new_v4().to_string());
    inner
}
