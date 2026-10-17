//! LSP completion definitions and helpers.

use super::*;
use lsp_types::CompletionItem;

/// Simplified completion item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItemInfo {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text: Option<String>,
}

impl From<CompletionItem> for CompletionItemInfo {
    fn from(item: CompletionItem) -> Self {
        Self {
            label: item.label,
            kind: item.kind.map(|k| format!("{:?}", k)),
            detail: item.detail,
            documentation: item.documentation.map(|d| match d {
                lsp_types::Documentation::String(s) => s,
                lsp_types::Documentation::MarkupContent(mc) => mc.value,
            }),
            insert_text: item.insert_text,
        }
    }
}
