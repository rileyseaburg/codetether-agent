//! Format workspace results for tool output.
use codetether_lsp_core::SymbolInfo;
pub(super) fn format(symbols: Vec<SymbolInfo>) -> String {
    if symbols.is_empty() {
        "No symbols found matching query".to_string()
    } else {
        let mut out = format!("Workspace symbols ({}):\n\n", symbols.len());
        for sym in symbols {
            out.push_str(&format!("  {} [{}]", sym.name, sym.kind));
            if let Some(uri) = sym.uri {
                out.push_str(&format!(" - {}", uri.trim_start_matches("file://")));
            }
            out.push('\n');
        }
        out
    }
}
