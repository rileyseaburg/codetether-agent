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

pub(super) fn revised(mut args: Value, arguments: Value, approval_id: &str) -> Value {
    if let (Some(original), Some(revised)) = (args.as_object_mut(), arguments.as_object()) {
        for (key, value) in revised {
            original.insert(key.clone(), value.clone());
        }
    }
    with_approval(args, approval_id)
}
