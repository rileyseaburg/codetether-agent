//! Symbol result parsing.
use std::path::PathBuf;

use crate::SymbolEntry;

/// Parses the textual workspace-symbol response from the LSP tool.
/// # Arguments
/// * `text` - Tool output with two header lines followed by symbol rows.
/// # Returns
/// Parsed entries; malformed rows are skipped.
/// # Examples
/// ```
/// assert!(codetether_symbol_search::parse::output("No symbols found").is_empty());
/// ```
pub fn output(text: &str) -> Vec<SymbolEntry> {
    text.lines().skip(2).filter_map(line).collect()
}

fn line(value: &str) -> Option<SymbolEntry> {
    let (identity, path) = value.trim().split_once(" - ")?;
    let (name, kind) = identity.rsplit_once(" [")?;
    Some(SymbolEntry {
        name: name.to_string(),
        kind: kind.strip_suffix(']')?.to_string(),
        path: PathBuf::from(path),
        uri: Some(format!("file://{path}")),
        line: None,
        container: None,
    })
}

#[cfg(test)]
mod tests {
    use super::output;

    #[test]
    fn parses_workspace_symbol_output() {
        let entries = output("Workspace symbols (1):\n\n  capture [Function] - /repo/pay.rs\n");
        assert_eq!(entries[0].name, "capture");
        assert_eq!(entries[0].kind, "Function");
        assert_eq!(entries[0].path.to_string_lossy(), "/repo/pay.rs");
    }

    #[test]
    fn ignores_empty_result_message() {
        assert!(output("No symbols found matching query").is_empty());
    }
}
