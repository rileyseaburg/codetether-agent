use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{Value, json};

use super::input::KilnPluginInput;
use super::runner;
use crate::tool::{Tool, ToolResult, tool_output_budget};

const DEFAULT_TIMEOUT_SECS: u64 = 10;
const MAX_TIMEOUT_SECS: u64 = 60;

pub struct KilnPluginTool {
    root: PathBuf,
}

impl KilnPluginTool {
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }
}

impl Default for KilnPluginTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for KilnPluginTool {
    fn id(&self) -> &str {
        "kiln_plugin"
    }

    fn name(&self) -> &str {
        "Kiln Plugin"
    }

    fn description(&self) -> &str {
        "Execute a Kiln plugin hook from inline `.kl` source or a `.kl` file path."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to a Kiln .kl plugin file"
                },
                "source": {
                    "type": "string",
                    "description": "Inline Kiln plugin source; used instead of path when provided"
                },
                "hook": {
                    "type": "string",
                    "description": "Top-level Kiln function to call"
                },
                "args": {
                    "type": "array",
                    "description": "JSON arguments converted to Kiln values",
                    "items": {}
                },
                "timeout_secs": {
                    "type": "integer",
                    "description": "Maximum wall-clock seconds to wait for the hook; capped at 60"
                }
            },
            "required": ["hook"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let input: KilnPluginInput = match serde_json::from_value(args) {
            Ok(input) => input,
            Err(error) => {
                return Ok(ToolResult::structured_error(
                    "invalid_params",
                    self.id(),
                    &format!("Invalid kiln_plugin params: {error}"),
                    None,
                    Some(
                        json!({"source": "fn validate() { return Ok(\"ok\") }", "hook": "validate"}),
                    ),
                ));
            }
        };
        if !input.has_source() && !input.has_path() {
            return Ok(ToolResult::structured_error(
                "missing_field",
                self.id(),
                "kiln_plugin requires either source or path",
                Some(vec!["source|path"]),
                Some(json!({"source": "fn validate() { return Ok(\"ok\") }", "hook": "validate"})),
            ));
        }

        let (source_name, source) = match load_source(&input, &self.root).await {
            Ok(source) => source,
            Err(error) => return Ok(ToolResult::error(error.to_string())),
        };
        let hook = input.hook;
        let values = input.args;
        let timeout = Duration::from_secs(
            input
                .timeout_secs
                .unwrap_or(DEFAULT_TIMEOUT_SECS)
                .min(MAX_TIMEOUT_SECS),
        );
        let task =
            tokio::task::spawn_blocking(move || runner::run(source_name, source, hook, values));
        let result = match tokio::time::timeout(timeout, task).await {
            Ok(joined) => joined.map_err(|join_error| {
                if join_error.is_panic() {
                    anyhow::anyhow!("Kiln plugin task panicked: {join_error}")
                } else {
                    anyhow::anyhow!("Kiln plugin task join failed: {join_error}")
                }
            })?,
            Err(_) => {
                return Ok(ToolResult::error(format!(
                    "Kiln plugin timed out after {} second(s)",
                    timeout.as_secs()
                )));
            }
        };

        match result {
            Ok(outcome) => Ok(ToolResult {
                output: outcome.output,
                success: outcome.success,
                metadata: [("value".to_string(), outcome.value)].into(),
            }
            .truncate_to(tool_output_budget())),
            Err(error) => Ok(ToolResult::error(format!("Kiln plugin failed: {error}"))
                .truncate_to(tool_output_budget())),
        }
    }
}

async fn load_source(input: &KilnPluginInput, root: &Path) -> Result<(String, String)> {
    if let Some(source) = input.source.as_deref().filter(|source| !source.is_empty()) {
        return Ok(("inline.kiln".to_string(), source.to_string()));
    }

    let path = input.path.as_deref().context("missing Kiln plugin path")?;
    let path = resolve_plugin_path(root, path).await?;
    let source = tokio::fs::read_to_string(&path)
        .await
        .with_context(|| format!("Failed to read Kiln plugin file: {}", path.display()))?;
    Ok((path.display().to_string(), source))
}

async fn resolve_plugin_path(root: &Path, raw_path: &str) -> Result<PathBuf> {
    let root = tokio::fs::canonicalize(root)
        .await
        .with_context(|| format!("Failed to resolve Kiln plugin root: {}", root.display()))?;
    let raw = Path::new(raw_path);
    let candidate = if raw.is_absolute() {
        raw.to_path_buf()
    } else {
        root.join(raw)
    };
    let candidate = tokio::fs::canonicalize(&candidate).await.with_context(|| {
        format!(
            "Failed to resolve Kiln plugin file: {}",
            candidate.display()
        )
    })?;

    if !candidate.starts_with(&root) {
        anyhow::bail!(
            "Kiln plugin path '{}' escapes workspace root '{}'",
            candidate.display(),
            root.display()
        );
    }
    if candidate.extension().and_then(|ext| ext.to_str()) != Some("kl") {
        anyhow::bail!(
            "Kiln plugin path '{}' must use the .kl extension",
            candidate.display()
        );
    }
    Ok(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::ToolRegistry;
    use serde_json::json;

    #[tokio::test]
    async fn executes_inline_kiln_hook_through_tool_trait() {
        let tool = KilnPluginTool::new();
        let result = tool
            .execute(json!({
                "source": r#"
fn validate(name) {
    println("kiln saw " + name)
    return Ok("hello " + name)
}
"#,
                "hook": "validate",
                "args": ["codetether"]
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.output.contains("kiln saw codetether"));
        assert!(result.output.contains("Ok(hello codetether)"));
        assert_eq!(
            result.metadata.get("value"),
            Some(&json!({"ok": "hello codetether"}))
        );
    }

    #[tokio::test]
    async fn kiln_err_result_becomes_failed_tool_result() {
        let tool = KilnPluginTool::new();
        let result = tool
            .execute(json!({
                "source": r#"fn validate() { return Err("feature rejected") }"#,
                "hook": "validate"
            }))
            .await
            .unwrap();

        assert!(!result.success);
        assert!(result.output.contains("Err(\"feature rejected\")"));
        assert_eq!(
            result.metadata.get("value"),
            Some(&json!({"err": "feature rejected"}))
        );
    }

    #[tokio::test]
    async fn executes_kiln_plugin_from_project_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("feature.kl");
        tokio::fs::write(
            &path,
            r#"fn validate(feature) { return Ok("accepted " + feature) }"#,
        )
        .await
        .unwrap();

        let tool = KilnPluginTool::with_root(dir.path().to_path_buf());
        let result = tool
            .execute(json!({
                "path": "feature.kl",
                "hook": "validate",
                "args": ["rust-project"]
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert_eq!(
            result.metadata.get("value"),
            Some(&json!({"ok": "accepted rust-project"}))
        );
    }

    #[tokio::test]
    async fn rejects_project_file_escape() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("workspace");
        tokio::fs::create_dir_all(&root).await.unwrap();
        let outside = dir.path().join("outside.kl");
        tokio::fs::write(&outside, r#"fn validate() { return Ok("bad") }"#)
            .await
            .unwrap();

        let tool = KilnPluginTool::with_root(root);
        let result = tool
            .execute(json!({
                "path": "../outside.kl",
                "hook": "validate"
            }))
            .await
            .unwrap();

        assert!(!result.success);
        assert!(result.output.contains("escapes workspace root"));
    }

    #[tokio::test]
    async fn invalid_params_return_tool_error() {
        let tool = KilnPluginTool::new();
        let result = tool.execute(json!({"hook": 42})).await.unwrap();

        assert!(!result.success);
        assert!(result.output.contains("Invalid kiln_plugin params"));
    }

    #[test]
    fn default_registry_exposes_kiln_plugin() {
        assert!(ToolRegistry::with_defaults().contains("kiln_plugin"));
    }
}
