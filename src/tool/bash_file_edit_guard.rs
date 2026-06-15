//! Guard that blocks file-mutating shell commands.
//!
//! Editing files through `cat > file`, here-docs, `python -c`/`python -` with
//! `open(..., "w")`, `tee`, or `sed -i` bypasses the structured edit tools and
//! hides the actual change. This guard detects those patterns so the bash tool
//! can steer the agent back to the `edit`/`multiedit` tools.

/// Returns a rejection reason if `command` writes file contents inline.
///
/// Returns `None` for ordinary shell usage (including read-only `cat file`).
///
/// # Examples
///
/// ```
/// use codetether_agent::tool::bash_file_edit_guard::file_edit_guard_reason;
///
/// assert!(file_edit_guard_reason("cat > src/x.rs <<'EOF'\n").is_some());
/// assert!(file_edit_guard_reason("cat src/x.rs").is_none());
/// ```
pub fn file_edit_guard_reason(command: &str) -> Option<&'static str> {
    let lower = command.to_ascii_lowercase();

    if lower.contains("<<") && (lower.contains("cat ") || lower.contains("tee ")) {
        return Some("here-doc file writes are blocked; use the edit or multiedit tool");
    }
    if lower.contains("cat >") || lower.contains("cat >>") {
        return Some("`cat >` file writes are blocked; use the edit or multiedit tool");
    }
    if lower.contains("tee ") {
        return Some("`tee` file writes are blocked; use the edit or multiedit tool");
    }
    if (lower.contains("python") || lower.contains("python3"))
        && (lower.contains(".write(")
            || lower.contains("open(") && lower.contains("'w'")
            || lower.contains("open(") && lower.contains("\"w\""))
    {
        return Some("editing files via python is blocked; use the edit or multiedit tool");
    }
    if lower.contains("sed -i") || lower.contains("sed --in-place") {
        return Some("`sed -i` edits are blocked; use the edit or multiedit tool");
    }
    None
}

/// Returns a structured rejection [`ToolResult`](crate::tool::ToolResult) when
/// `command` mutates files inline, or `None` when the command may run.
pub fn file_edit_guard_result(command: &str) -> Option<crate::tool::ToolResult> {
    let reason = file_edit_guard_reason(command)?;
    Some(crate::tool::ToolResult::structured_error(
        "FILE_EDIT_VIA_BASH_BLOCKED",
        "bash",
        reason,
        None,
        Some(serde_json::json!({
            "use_instead": ["edit", "multiedit", "write"],
            "why": "Inline shell file writes hide the actual change and bypass review.",
        })),
    ))
}

#[cfg(test)]
#[path = "bash_file_edit_guard_tests.rs"]
mod bash_file_edit_guard_tests;
