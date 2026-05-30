//! Edit tool: replace strings in files.

#[path = "edit/args.rs"]
mod args;
#[path = "edit/diff.rs"]
mod diff;
#[path = "edit/matcher.rs"]
mod matcher;
#[path = "edit/morph.rs"]
mod morph;
#[path = "edit/result.rs"]
mod result;
#[path = "edit/schema.rs"]
mod schema;

use self::args::EditArgs;
use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use tokio::fs;

/// Edit files by replacing strings.
pub struct EditTool;

impl Default for EditTool {
    fn default() -> Self {
        Self::new()
    }
}

impl EditTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for EditTool {
    fn id(&self) -> &str {
        "edit"
    }
    fn name(&self) -> &str {
        "Edit File"
    }
    fn description(&self) -> &str {
        "edit(path, old_string, new_string, replace_all?) - Replace text with exact, whitespace-tolerant, or diagnostic fuzzy matching."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, raw: Value) -> Result<ToolResult> {
        let args = match EditArgs::parse(&raw) {
            Ok(args) => args,
            Err(_) => {
                return Ok(result::invalid(
                    "path",
                    "path is required",
                    json!({"path":"src/main.rs"}),
                ));
            }
        };
        let content = fs::read_to_string(args.path).await?;
        if let Some(updated) = morph::apply(&content, &args).await? {
            return Ok(result::preview(
                args.path, &content, &updated, &content, &updated, "morph",
            ));
        }
        let old = match args.old_string {
            Some(old) => old,
            None => {
                return Ok(result::invalid(
                    "old_string",
                    "old_string is required unless Morph handles the edit",
                    json!({"path":args.path,"old_string":"old","new_string":"new"}),
                ));
            }
        };
        let new = match args.new_string {
            Some(new) => new,
            None => {
                return Ok(result::invalid(
                    "new_string",
                    "new_string is required unless Morph handles the edit",
                    json!({"path":args.path,"old_string":old,"new_string":"new"}),
                ));
            }
        };
        let plan = match matcher::plan(&content, old, args.replace_all) {
            Ok(plan) => plan,
            Err(error) => return Ok(result::matcher_error(error)),
        };
        let (updated, replacements, backend) = matcher::apply(&content, plan, old, new);
        let mut preview = result::preview(args.path, old, new, &content, &updated, backend);
        preview
            .metadata
            .insert("replacements".into(), json!(replacements));
        Ok(preview)
    }
}
