use std::path::Path;

pub fn is_source(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("rs" | "ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs")
    )
}

pub fn code_lines(text: &str) -> usize {
    let mut count = 0usize;
    let mut in_block = false;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if in_block {
            if line.contains("*/") {
                in_block = false;
            }
            continue;
        }
        if line.starts_with("//") {
            continue;
        }
        if line.starts_with("/*") {
            if !line.contains("*/") {
                in_block = true;
            }
            continue;
        }
        count += 1;
    }
    count
}

pub fn code_line_set(text: &str) -> std::collections::HashSet<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(str::to_string)
        .collect()
}
