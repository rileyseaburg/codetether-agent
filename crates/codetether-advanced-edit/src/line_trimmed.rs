//! line trimmed matching strategy.

/// Match with trimmed lines
pub(super) fn line_trimmed_replacer(content: &str, find: &str) -> Vec<String> {
    let orig_lines: Vec<&str> = content.lines().collect();
    let mut search_lines: Vec<&str> = find.lines().collect();
    if search_lines.last() == Some(&"") {
        search_lines.pop();
    }
    let mut results = vec![];
    for i in 0..=orig_lines.len().saturating_sub(search_lines.len()) {
        let mut matches = true;
        for j in 0..search_lines.len() {
            if orig_lines.get(i + j).map(|l| l.trim()) != Some(search_lines[j].trim()) {
                matches = false;
                break;
            }
        }
        if matches {
            let matched: Vec<&str> = orig_lines[i..i + search_lines.len()].to_vec();
            results.push(matched.join("\n"));
        }
    }
    results
}
