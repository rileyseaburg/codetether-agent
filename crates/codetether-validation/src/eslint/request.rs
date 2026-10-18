//! Construct the existing sandbox ESLint request for supported source files.
use serde_json::{Value, json};
use std::path::Path;
pub(super) fn request(workspace_dir: &Path, path: &Path) -> Option<Value> {
    if !js_like(path) {
        return None;
    }
    let relative = relative_path(workspace_dir, path);
    let args = json!({
        "command": format!("npx --no-install eslint --format json {} 2>/dev/null || true", quote(&relative)),
        "cwd": workspace_dir.display().to_string()
    });
    Some(args)
}

fn js_like(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| matches!(ext, "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs"))
}

fn relative_path(workspace_dir: &Path, path: &Path) -> String {
    path.strip_prefix(workspace_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
