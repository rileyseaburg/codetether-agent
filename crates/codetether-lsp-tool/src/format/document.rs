//! Format document results for tool output.
use codetether_lsp_core::SymbolInfo;
pub(super) fn format(symbols: Vec<SymbolInfo>) -> String {
    if symbols.is_empty() {
        "No symbols found in document".to_string()
    } else {
        let mut out = format!("Document symbols ({}):\n\n", symbols.len());
        for sym in symbols {
            out.push_str(&format!("  {} [{}]", sym.name, sym.kind));
            if let Some(detail) = sym.detail {
                out.push_str(&format!(" - {}", detail));
            }
            out.push('\n');
        }
        out
    }
}
