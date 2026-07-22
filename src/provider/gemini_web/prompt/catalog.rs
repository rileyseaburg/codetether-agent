//! Canonical tool catalog supplied to Gemini Web as text protocol data.

use crate::provider::ToolDefinition;
use anyhow::{Result, bail};

const MAX_BYTES: usize = 128 * 1024;

pub(super) fn render(tools: &[ToolDefinition]) -> Result<String> {
    let payload = serde_json::to_string(tools)?;
    let safe = payload
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026");
    let rendered = format!("System: <available_tools>{safe}</available_tools>");
    if rendered.len() > MAX_BYTES {
        bail!(
            "Gemini Web tool catalog is {} bytes; maximum is {MAX_BYTES}",
            rendered.len()
        );
    }
    Ok(rendered)
}

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod tests;
