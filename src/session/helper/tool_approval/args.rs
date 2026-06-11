use serde_json::Value;

pub(super) fn with_approval(mut args: Value, approval_id: &str) -> Value {
    match args.as_object_mut() {
        Some(map) => {
            map.insert("approval_id".into(), Value::String(approval_id.to_string()));
            args
        }
        None => args,
    }
}
