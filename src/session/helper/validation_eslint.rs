//! Sandboxed ESLint diagnostics for validation prompts.

#[path = "validation_eslint_types.rs"]
mod types;

use crate::tool::{Tool, bash::BashTool};
use serde_json::json;
use std::path::Path;

pub(super) async fn collect(workspace_dir: &Path, path: &Path) -> Vec<String> {
    if !js_like(path) {
        return Vec::new();
    }
    let relative = relative_path(workspace_dir, path);
    let args = json!({
        "command": format!("npx --no-install eslint --format json {} 2>/dev/null || true", quote(&relative)),
        "cwd": workspace_dir.display().to_string()
    });
    if crate::runtime_policy::evaluate_tool_invocation("bash", &args)
        .await
        .is_some()
    {
        return Vec::new();
    }
    let Ok(result) = BashTool::new().execute(args).await else {
        return Vec::new();
    };
    let Ok(reports) = serde_json::from_str::<Vec<types::FileReport>>(&result.output) else {
        return Vec::new();
    };
    reports
        .into_iter()
        .flat_map(types::FileReport::render)
        .collect()
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
