//! Find candidate blocks using the first and last search lines.
pub(super) fn block_anchor_replacer(content: &str, find: &str) -> Vec<String> {
    let orig_lines: Vec<&str> = content.lines().collect();
    let mut search_lines: Vec<&str> = find.lines().collect();
    if search_lines.len() < 3 {
        return vec![];
    }
    if search_lines.last() == Some(&"") {
        search_lines.pop();
    }
    let first = search_lines[0].trim();
    let last = match search_lines.last() {
        Some(l) => l.trim(),
        None => return vec![],
    };
    let mut candidates = vec![];
    for i in 0..orig_lines.len() {
        if orig_lines[i].trim() != first {
            continue;
        }
        for j in (i + 2)..orig_lines.len() {
            if orig_lines[j].trim() == last {
                candidates.push((i, j));
                break;
            }
        }
    }
    if candidates.is_empty() {
        return vec![];
    }

    super::block_best::select(&orig_lines, &search_lines, candidates)
}
