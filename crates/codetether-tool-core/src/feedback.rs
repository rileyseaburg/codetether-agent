//! Model-facing feedback for tool-call results.

use std::path::Path;

/// Render a concise tool result envelope for the next model step.
/// Render model-facing status and output text.
///
/// # Arguments
/// * `tool` - Tool identifier.
/// * `success` - Execution outcome.
/// * `output` - Tool text, trimmed before rendering.
///
/// # Returns
/// The normalized metadata, provider parts, or rendered text described above.
///
/// # Examples
/// ```
/// let text = codetether_tool_core::feedback::render("read", true, "ok");
/// assert!(text.contains("status: success"));
/// ```
pub fn render(tool: &str, success: bool, output: &str) -> String {
    render_with_dir(tool, success, None, output)
}

/// Render a tool result envelope with optional working-directory context.
/// Render model-facing status, directory, and output text.
///
/// # Arguments
/// * `tool` - Tool identifier.
/// * `success` - Execution outcome.
/// * `dir` - Optional working-directory context.
/// * `output` - Tool text, trimmed before rendering.
///
/// # Returns
/// The normalized metadata, provider parts, or rendered text described above.
///
/// # Examples
/// ```
/// let text = codetether_tool_core::feedback::render_with_dir("read", false, None, "");
/// assert!(text.contains("<empty tool output>"));
/// ```
pub fn render_with_dir(tool: &str, success: bool, dir: Option<&Path>, output: &str) -> String {
    let status = if success { "success" } else { "failure" };
    let body = output.trim();
    let body = if body.is_empty() {
        "<empty tool output>"
    } else {
        body
    };
    let mut rendered = format!("Tool call feedback\n- tool: {tool}\n- status: {status}");
    if let Some(dir) = dir {
        rendered.push_str(&format!("\n- working_dir: {}", dir.display()));
    }
    if !success {
        rendered.push_str(
            "\n- next: inspect the error, correct the arguments or approach, \
             and avoid repeating the same failed call unchanged.",
        );
    }
    rendered.push_str("\n\nOutput:\n");
    rendered.push_str(body);
    rendered
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn includes_status_and_output() {
        let out = render("bash", false, "permission denied");
        assert!(out.contains("- tool: bash"));
        assert!(out.contains("- status: failure"));
        assert!(out.contains("permission denied"));
    }
}
