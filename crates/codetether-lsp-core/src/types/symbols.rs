//! LSP symbols definitions and helpers.

use super::*;
use lsp_types::{DocumentSymbol, SymbolInformation};

/// Simplified symbol info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}

impl From<DocumentSymbol> for SymbolInfo {
    fn from(sym: DocumentSymbol) -> Self {
        Self {
            name: sym.name,
            kind: format!("{:?}", sym.kind),
            detail: sym.detail,
            uri: None,
            range: Some(RangeInfo::from(sym.range)),
            container_name: None,
        }
    }
}

impl From<SymbolInformation> for SymbolInfo {
    fn from(sym: SymbolInformation) -> Self {
        Self {
            name: sym.name,
            kind: format!("{:?}", sym.kind),
            detail: None,
            uri: Some(sym.location.uri.to_string()),
            range: Some(RangeInfo::from(sym.location.range)),
            container_name: sym.container_name,
        }
    }
}
