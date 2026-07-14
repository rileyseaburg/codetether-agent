use anyhow::{Result, bail};
use serde_json::Value;

pub fn reject_unknown_path_fields(value: &Value, specs: &[String], prefix: String) -> Result<()> {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                let path = join(&prefix, key);
                if is_path_like(key) && !declared(&path, specs) {
                    bail!("undeclared filesystem path field: {path}");
                }
                reject_unknown_path_fields(child, specs, path)?;
            }
        }
        Value::Array(items) => {
            for (i, child) in items.iter().enumerate() {
                reject_unknown_path_fields(child, specs, join(&prefix, &i.to_string()))?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn declared(path: &str, specs: &[String]) -> bool {
    specs
        .iter()
        .any(|spec| spec == path || spec.starts_with(&format!("{path}.")))
}

pub fn contains_path_like(value: &Value) -> bool {
    reject_unknown_path_fields(value, &[], String::new()).is_err()
}

fn is_path_like(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    matches!(lower.as_str(), "file" | "cwd" | "dir" | "directory") || lower.contains("path")
}

fn join(prefix: &str, key: &str) -> String {
    if prefix.is_empty() {
        key.to_string()
    } else {
        format!("{prefix}.{key}")
    }
}
