//! Explicit execution targets added before approval scope calculation.

use serde_json::Value;
use std::path::Path;

pub(crate) fn bind_workspace(tool: &str, mut input: Value, workspace: &Path) -> Value {
    let Some(map) = input.as_object_mut() else {
        return input;
    };
    if matches!(
        tool,
        "read" | "write" | "edit" | "list" | "fileinfo" | "headtail"
    ) {
        root_path(map, "path", workspace);
        return input;
    }
    let (field, alias) = match tool {
        "bash" | "git" => ("cwd", None),
        "exec_command" => ("workdir", Some("cwd")),
        _ => return input,
    };
    if !map.contains_key(field) && alias.is_none_or(|key| !map.contains_key(key)) {
        map.insert(field.into(), Value::String(workspace.display().to_string()));
    }
    input
}

fn root_path(map: &mut serde_json::Map<String, Value>, field: &str, workspace: &Path) {
    let Some(path) = map.get(field).and_then(Value::as_str) else {
        return;
    };
    let path = Path::new(path);
    if path.is_relative() {
        map.insert(
            field.into(),
            Value::String(workspace.join(path).display().to_string()),
        );
    }
}

#[cfg(test)]
#[test]
fn binds_exec_target_before_review() {
    let input = bind_workspace(
        "exec_command",
        serde_json::json!({"cmd": "true"}),
        Path::new("root"),
    );
    assert_eq!(input["workdir"], "root");
}
