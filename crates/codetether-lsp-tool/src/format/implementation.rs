//! Format implementation results for tool output.
use codetether_lsp_core::LocationInfo;
pub(super) fn format(locations: Vec<LocationInfo>) -> String {
    if locations.is_empty() {
        "No implementations found".to_string()
    } else {
        let mut out = format!("Found {} implementation(s):\n\n", locations.len());
        for loc in locations {
            let uri = loc.uri;
            let range = loc.range;
            out.push_str(&format!(
                "  {}:{}:{}\n",
                uri.trim_start_matches("file://"),
                range.start.line + 1,
                range.start.character + 1
            ));
        }
        out
    }
}
