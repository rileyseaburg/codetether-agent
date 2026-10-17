//! extract python imports for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_python_imports(
    line: &str,
    imported_modules: &mut Vec<String>,
    imported_symbols: &mut Vec<String>,
) {
    if let Some(rest) = line.strip_prefix("import ") {
        for module in rest.split(',') {
            let module = module.trim();
            let module = module.split_whitespace().next().unwrap_or("");
            if module.is_empty() {
                continue;
            }
            imported_modules.push(module.to_string());
            if let Some(name) = module.rsplit('.').next().and_then(sanitize_identifier) {
                imported_symbols.push(name);
            }
        }
        return;
    }

    let Some(rest) = line.strip_prefix("from ") else {
        return;
    };
    let Some((module, names)) = rest.split_once(" import ") else {
        return;
    };
    let module = module.trim();
    if !module.is_empty() {
        imported_modules.push(module.to_string());
    }
    for name in names.split(',') {
        let name = name.trim();
        let alias_free = name.split(" as ").next().unwrap_or(name).trim();
        if let Some(clean) = sanitize_identifier(alias_free) {
            imported_symbols.push(clean);
        }
    }
}
