use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{Value, json};

use super::input::KilnPluginInput;
use super::runner;
use crate::tool::{Tool, ToolResult};

pub struct KilnPluginTool;

impl KilnPluginTool {
    pub fn new() -> Self {
        Self
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
                }
            },
            "required": ["hook"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let input: KilnPluginInput = serde_json::from_value(args).context("Invalid params")?;
        if !input.has_source() && !input.has_path() {
            return Ok(ToolResult::structured_error(
                "missing_field",
                self.id(),
                "kiln_plugin requires either source or path",
                Some(vec!["source|path"]),
                Some(json!({"source": "fn validate() { return Ok(\"ok\") }", "hook": "validate"})),
            ));
        }

        let (source_name, source) = load_source(&input).await?;
        let hook = input.hook;
        let values = input.args;
        let result =
            tokio::task::spawn_blocking(move || runner::run(source_name, source, hook, values))
                .await
                .context("Kiln plugin task panicked")?;

        match result {
            Ok(outcome) => Ok(ToolResult {
                output: outcome.output,
                success: outcome.success,
                metadata: [("value".to_string(), outcome.value)].into(),
            }),
            Err(error) => Ok(ToolResult::error(format!("Kiln plugin failed: {error}"))),
        }
    }
}

async fn load_source(input: &KilnPluginInput) -> Result<(String, String)> {
    if let Some(source) = input.source.as_deref().filter(|source| !source.is_empty()) {
        return Ok(("inline.kiln".to_string(), source.to_string()));
    }

    let path = input.path.as_deref().context("missing Kiln plugin path")?;
    let source = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Failed to read Kiln plugin file: {path}"))?;
    Ok((path.to_string(), source))
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

        let tool = KilnPluginTool::new();
        let result = tool
            .execute(json!({
                "path": path.to_string_lossy(),
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

    #[test]
    fn default_registry_exposes_kiln_plugin() {
        assert!(ToolRegistry::with_defaults().contains("kiln_plugin"));
    }
}
