//! Python execution rejection, independent of file-edit and approval policy.

use crate::tool::ToolResult;

mod detect;

pub(super) fn result(tool: &str, command: &str) -> Option<ToolResult> {
    detect::detected(command).then(|| {
        ToolResult::structured_error(
            "PYTHON_EXECUTION_BLOCKED",
            tool,
            "Python execution (python, python3, and versioned interpreters) is blocked; \
             use TetherScript or native tools instead.",
            None,
            Some(serde_json::json!({"use_instead": ["tetherscript_plugin", "native tools"]})),
        )
    })
}

#[cfg(test)]
mod tests;
