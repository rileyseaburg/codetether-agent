//! Surface unavailable automatic verification without misreporting the patch outcome.

use crate::session::helper::tool_policy::ToolTuple;
use serde_json::json;

pub(in crate::session::helper) fn annotate(
    mut result: ToolTuple,
    warnings: Vec<String>,
) -> ToolTuple {
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
