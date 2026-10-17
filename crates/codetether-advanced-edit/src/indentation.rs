//! indentation matching strategy.

/// Indentation flexible matching
pub(super) fn indentation_flexible_replacer(content: &str, find: &str) -> Vec<String> {
    let remove_indent = |s: &str| {
        let lines: Vec<&str> = s.lines().collect();
        let non_empty: Vec<_> = lines.iter().filter(|l| !l.trim().is_empty()).collect();
        if non_empty.is_empty() {
            return s.to_string();
        }
        let min_indent = non_empty
            .iter()
            .map(|l| l.len() - l.trim_start().len())
            .min()
            .unwrap_or(0);
        lines
            .iter()
            .map(|l| {
                if l.len() >= min_indent {
                    &l[min_indent..]
                } else {
                    *l
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let norm_find = remove_indent(find);
    let lines: Vec<&str> = content.lines().collect();
    let find_lines: Vec<&str> = find.lines().collect();
    let mut results = vec![];
    for i in 0..=lines.len().saturating_sub(find_lines.len()) {
        let block = lines[i..i + find_lines.len()].join("\n");
        if remove_indent(&block) == norm_find {
            results.push(block);
        }
    }
    results
}
