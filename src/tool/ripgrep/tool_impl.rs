//! `Tool` trait implementation for [`RipgrepTool`].

use super::{RgArgs, RipgrepTool, command, exec, render, schema};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

#[async_trait]
impl Tool for RipgrepTool {
    fn id(&self) -> &str {
        "rg"
    }

    fn name(&self) -> &str {
        "Ripgrep"
    }

    fn description(&self) -> &str {
        "rg(pattern, paths?, glob?, fixed_strings?, case_insensitive?, files_with_matches?, context_lines?, max_count?, limit?) - Search with the real ripgrep binary. Full regex alternation (a|b) and --glob exclude filters. Prefer this over grep for regex patterns."
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let args: RgArgs = match serde_json::from_value(args) {
            Ok(parsed) => parsed,
            Err(error) => return Ok(invalid(&format!("could not parse arguments: {error}"))),
        };
        if args.pattern.is_empty() {
            return Ok(invalid("pattern is required and must not be empty"));
        }
        let flags = command::build(&args);
        let output = exec::run(&flags, self.root(), args.timeout()).await?;
        Ok(render::render(output, args.limit()))
    }
}

fn invalid(message: &str) -> ToolResult {
    ToolResult::structured_error(
        "INVALID_ARGUMENT",
        "rg",
        message,
        Some(vec!["pattern"]),
        Some(json!({"pattern": "fn main", "glob": ["*.rs"]})),
    )
}
