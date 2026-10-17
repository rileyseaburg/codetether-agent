//! Format completion results for tool output.
use codetether_lsp_core::CompletionItemInfo;
pub(super) fn format(items: Vec<CompletionItemInfo>) -> String {
    if items.is_empty() {
        "No completions available".to_string()
    } else {
        let mut out = format!("Completions ({}):\n\n", items.len());
        for item in items {
            out.push_str(&format!("  {}", item.label));
            if let Some(kind) = item.kind {
                out.push_str(&format!(" [{}]", kind));
            }
            if let Some(detail) = item.detail {
                out.push_str(&format!(" - {}", detail));
            }
            out.push('\n');
        }
        out
    }
}
