use super::inspect;
use anyhow::{Result, bail};
use serde_json::Value;

pub fn field_specs(tool: &str, args: &Value) -> Result<Vec<String>> {
    let specs = match tool {
        "read" | "write" | "list" | "grep" | "codesearch" => fields(&["path"]),
        "bash" => fields(&["cwd"]),
        "edit" => fields(&["filePath"]),
        "patch" => fields(&["file"]),
        "glob" => fields(&["pattern"]),
        "confirm_edit" => fields(&["path"]),
        "multiedit" | "confirm_multiedit" => edit_specs(args),
        _ if inspect::contains_path_like(args) => {
            bail!("tool {tool} has undeclared filesystem path field")
        }
        _ => Vec::new(),
    };
    Ok(specs)
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
