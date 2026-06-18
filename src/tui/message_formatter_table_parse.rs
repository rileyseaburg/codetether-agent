//! Parsing helpers for markdown tables: row detection and cell splitting.

/// Split a markdown table row into trimmed cell strings.
///
/// Strips a single leading/trailing pipe, then splits on unescaped `|`.
pub(crate) fn split_cells(row: &str) -> Vec<String> {
    let trimmed = row.trim();
    let inner = trimmed
        .strip_prefix('|')
        .unwrap_or(trimmed)
        .strip_suffix('|')
        .unwrap_or_else(|| trimmed.strip_prefix('|').unwrap_or(trimmed));
    inner.split('|').map(|c| c.trim().to_string()).collect()
}

/// True when a line looks like a table row (contains a pipe with content).
pub(crate) fn is_table_row(line: &str) -> bool {
    let t = line.trim();
    t.contains('|') && !t.is_empty()
}

/// True when a line is a GFM separator row, e.g. `|---|:--:|---:|`.
pub(crate) fn is_separator_row(line: &str) -> bool {
    let cells = split_cells(line);
    if cells.is_empty() {
        return false;
    }
    cells.iter().all(|c| {
        let c = c.trim();
        !c.is_empty() && c.chars().all(|ch| ch == '-' || ch == ':') && c.contains('-')
    })
}
