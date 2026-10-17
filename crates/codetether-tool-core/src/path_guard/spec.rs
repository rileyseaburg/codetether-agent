use super::inspect;
use anyhow::{Result, bail};
use serde_json::Value;

pub fn field_specs(tool: &str, args: &Value) -> Result<Vec<String>> {
    let specs = match tool {
        "read" | "write" | "list" | "grep" | "codesearch" | "tree" | "fileinfo" | "file_info"
        | "headtail" | "head_tail" => fields(&["path"]),
        "bash" => fields(&["cwd"]),
        "git" => git_specs(args),
        "edit" | "confirm_edit" => fields(&["path"]),
        "apply_patch" | "patch" => Vec::new(),
        "glob" => fields(&["pattern"]),
        "lsp" => fields(&["file_path", "path"]),
        "diff" => fields(&["file1", "file2"]),
        "multiedit" | "confirm_multiedit" => edit_specs(args),
        _ if inspect::contains_path_like(args) => {
            bail!("tool {tool} has undeclared filesystem path field")
        }
        _ => Vec::new(),
    };
    Ok(specs)
}

fn git_specs(args: &Value) -> Vec<String> {
    let mut specs = fields(&["cwd", "path"]);
    if let Some(paths) = args.get("paths").and_then(Value::as_array) {
        specs.extend((0..paths.len()).map(|i| format!("paths.{i}")));
    }
    specs
}

fn fields(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

fn edit_specs(args: &Value) -> Vec<String> {
    args.get("edits")
        .and_then(Value::as_array)
        .map(|edits| {
            (0..edits.len())
                .map(|i| format!("edits.{i}.file"))
                .collect()
        })
        .unwrap_or_default()
}
