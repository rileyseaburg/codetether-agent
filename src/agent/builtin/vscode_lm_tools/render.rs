//! Render VS Code language model tools as compact prompt guidance.

use std::{fs, path::Path};

use serde_json::Value;

use super::discover::package_json_from;
use super::tool_line::render_tool;

const MAX_TOOLS: usize = 8;

pub(super) fn render_section(cwd: &Path) -> String {
    let Some(package) = package_json_from(cwd) else {
        return String::new();
    };
    let Ok(content) = fs::read_to_string(&package) else {
        return String::new();
    };
    let Ok(root) = serde_json::from_str::<Value>(&content) else {
        return String::new();
    };
    let tools = super::summary::summaries(&root);
    if tools.is_empty() {
        return String::new();
    }
    let mut out = format!(
        "\n\n## Workspace VS Code Language Model Tools\n\n\
         `{}` declares VS Code/Copilot language model tools. \
         Treat these as extension contracts, not native CodeTether tools. \
         Keep package schemas, implementations, and docs aligned. \
         For model-picker issues, inspect `vscode.lm.selectChatModels()`, \
         token filters, Copilot permissions, and saved model settings.\n\n",
        package.display()
    );
    for tool in tools.iter().take(MAX_TOOLS) {
        out.push_str(&render_tool(tool));
    }
    if tools.len() > MAX_TOOLS {
        out.push_str(&format!("- ... {} more tool(s)\n", tools.len() - MAX_TOOLS));
    }
    out
}
