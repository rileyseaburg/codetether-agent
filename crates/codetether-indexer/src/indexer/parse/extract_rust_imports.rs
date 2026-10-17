//! extract rust imports for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_rust_imports(
    line: &str,
    imported_modules: &mut Vec<String>,
    imported_symbols: &mut Vec<String>,
) {
    let normalized = strip_prefixes(line, &["pub "]);
    let Some(spec) = normalized.strip_prefix("use ") else {
        return;
    };
    let spec = spec.trim_end_matches(';').trim();
    if spec.is_empty() {
        return;
    }

    imported_modules.push(spec.to_string());
    for segment in spec.split(&['{', '}', ','][..]) {
        let segment = segment.trim();
        if segment.is_empty() {
            continue;
        }

        let alias_free = segment.split(" as ").next().unwrap_or(segment).trim();
        let last = alias_free.rsplit("::").next().unwrap_or(alias_free).trim();
        if last.is_empty() || matches!(last, "self" | "super" | "crate" | "*") {
            continue;
        }
        if let Some(name) = sanitize_identifier(last) {
            imported_symbols.push(name);
        }
    }
}
