//! Format hover results for tool output.
use codetether_lsp_core::RangeInfo;
pub(super) fn format(contents: String, range: Option<RangeInfo>) -> String {
    let mut out = "Hover information:\n\n".to_string();
    out.push_str(&contents);
    if let Some(r) = range {
        out.push_str(&format!(
            "\n\nRange: line {}-{}, col {}-{}",
            r.start.line + 1,
            r.end.line + 1,
            r.start.character + 1,
            r.end.character + 1
        ));
    }
    out
}
