//! Identification of tool invocations that can cross a network boundary.

#[path = "network_governed_actions.rs"]
mod actions;
#[path = "network_governed_always.rs"]
mod always;
#[path = "network_governed_collaboration.rs"]
mod collaboration;

use serde_json::Value;

pub(super) fn check(tool: &str, args: &Value) -> bool {
    if matches!(tool, "edit" | "multiedit") {
        return crate::tool::morph_backend::should_use_morph_backend();
    }
    if let Some(governed) = actions::check(tool, args) {
        return governed;
    }
    if tool == "browserctl" {
        return !matches!(
            args.get("action").and_then(Value::as_str),
            Some("health" | "detect" | "stop")
        );
    }
    if tool == "image" {
        return args
            .get("path")
            .and_then(Value::as_str)
            .is_some_and(|path| path.starts_with("http://") || path.starts_with("https://"));
    }
    always::check(tool)
        || collaboration::check(tool)
        || tool.starts_with("mcp:")
        || tool.starts_with("mcp__")
        || tool.starts_with("image_generation")
}
