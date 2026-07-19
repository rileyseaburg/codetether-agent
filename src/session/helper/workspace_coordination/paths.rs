//! Mutation-to-path mapping at the central tool boundary.

use serde_json::Value;
use std::path::PathBuf;

pub(super) fn mutation_paths(tool: &str, input: &Value) -> Option<Vec<PathBuf>> {
    match tool {
        "write" | "edit" => super::structured::single(input, "path"),
        "multiedit" => super::structured::many(input),
        "confirm_edit" if confirmed(input) => super::structured::single(input, "path"),
        "confirm_multiedit" if confirmed(input) => super::structured::many(input),
        "apply_patch" | "patch" if applies(input) => super::patch::paths(input),
        "git" if input["op"] == "commit" => Some(vec![PathBuf::new()]),
        "undo" if !input["preview"].as_bool().unwrap_or(false) => Some(vec![PathBuf::new()]),
        "bash" => shell(input, "command"),
        "exec_command" => shell(input, "cmd"),
        "write_stdin" if nonempty(input, "chars") => root(),
        "todo_write" | "tetherscript_plugin" => root(),
        "voice" if input["action"] == "speak" => root(),
        "browserctl" if input["action"] == "screenshot" => super::structured::single(input, "path"),
        "mcp" if input["action"] == "call_tool" => root(),
        "batch" => super::batch::paths(input),
        _ => None,
    }
}

fn confirmed(input: &Value) -> bool {
    input.get("confirm").and_then(Value::as_bool) == Some(true)
}

fn applies(input: &Value) -> bool {
    !["dry_run", "preview"]
        .into_iter()
        .any(|key| input.get(key).and_then(Value::as_bool) == Some(true))
}

fn shell(input: &Value, field: &str) -> Option<Vec<PathBuf>> {
    let command = input.get(field)?.as_str()?;
    (!super::shell::read_only(command)).then(|| vec![PathBuf::new()])
}

fn nonempty(input: &Value, field: &str) -> bool {
    input
        .get(field)
        .and_then(Value::as_str)
        .is_some_and(|value| !value.is_empty())
}

fn root() -> Option<Vec<PathBuf>> {
    Some(vec![PathBuf::new()])
}
