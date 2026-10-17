//! whitespace matching strategy.

/// Whitespace normalized matching
pub(super) fn whitespace_normalized_replacer(content: &str, find: &str) -> Vec<String> {
    let normalize = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
    let norm_find = normalize(find);
    let mut results = vec![];
    for line in content.lines() {
        if normalize(line) == norm_find {
            results.push(line.to_string());
        }
    }
    // Multi-line
    let find_lines: Vec<&str> = find.lines().collect();
    if find_lines.len() > 1 {
        let lines: Vec<&str> = content.lines().collect();
        for i in 0..=lines.len().saturating_sub(find_lines.len()) {
            let block = lines[i..i + find_lines.len()].join("\n");
            if normalize(&block) == norm_find {
                results.push(block);
            }
        }
    }
    results
}
