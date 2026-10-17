use serde_json::{Value, json};

pub(super) fn assistant(ids: &[&str]) -> Value {
    let content: Vec<_> = ids
        .iter()
        .map(|id| {
            json!({
                "toolUse": {"toolUseId": id, "name": "t", "input": {}}
            })
        })
        .collect();
    json!({"role": "assistant", "content": content})
}
