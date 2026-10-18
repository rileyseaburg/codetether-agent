//! Surface unavailable automatic verification without misreporting the patch outcome.

use codetether_tool_reporting::ToolTuple;
use serde_json::json;

/// Append verification warnings while preserving the tool's success status.
///
/// # Arguments
/// `result` is the execution output; `warnings` describes unavailable checks.
///
/// # Returns
/// The original result with warning text and structured metadata when needed.
///
/// # Examples
/// ```
/// let result = ("saved".into(), true, None);
/// assert_eq!(codetether_tool_approval::annotate(result.clone(), vec![]), result);
/// ```
pub fn annotate(mut result: ToolTuple, warnings: Vec<String>) -> ToolTuple {
    if warnings.is_empty() {
        return result;
    }
    let rendered = warnings
        .iter()
        .take(10)
        .map(|warning| format!("- {warning}"))
        .collect::<Vec<_>>()
        .join("\n");
    result.0.push_str(&format!(
        "\n\nWarning [LSP_PREAPPROVAL_UNAVAILABLE]: automatic code verification is incomplete; \
         do not treat this tool result as a clean diagnostic check.\n{rendered}"
    ));
    result.2.get_or_insert_with(Default::default).insert(
        "lsp_preapproval".into(),
        json!({"status": "unavailable", "code": "LSP_PREAPPROVAL_UNAVAILABLE", "warnings": warnings}),
    );
    result
}

#[cfg(test)]
#[path = "warnings_tests.rs"]
mod tests;
