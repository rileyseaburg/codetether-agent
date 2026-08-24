//! Runtime policy invocation arguments for special worker tasks.

use serde_json::Value;

pub(super) fn from_context(task: &Value, title: &str, context: &super::TaskContext) -> Value {
    let mut args = serde_json::json!({
        "task": sanitized(task),
        "title": title,
        "prompt": context.prompt,
        "agent": context.raw_agent,
        "model": context.selected_model,
        "__ct_session_id": context.resume_session_id.as_deref()
            .or(context.context_id.as_deref())
            .unwrap_or("a2a-special-task"),
    });
    let network = crate::tool::network_access::allowed();
    crate::tool::network_access::bind_trusted(&mut args, network);
    if let Some(value) = super::metadata_lookup(&context.metadata, "approval_id") {
        args["approval_id"] = value.clone();
    }
    args
}

fn sanitized(value: &Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.iter()
                .filter(|(key, _)| key.as_str() != "approval_id")
                .map(|(key, value)| (key.clone(), sanitized(value)))
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(sanitized).collect()),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => value.clone(),
    }
}

#[cfg(test)]
#[path = "task_policy_args_tests.rs"]
mod tests;
