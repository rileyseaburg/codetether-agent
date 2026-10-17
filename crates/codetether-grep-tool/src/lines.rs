//! lines for bounded grep searches.
use super::*;
pub(super) fn scan(
    content: &str,
    path: &std::path::Path,
    regex: &Regex,
    limit: usize,
    results: &mut Vec<String>,
) {
    for (line_num, line) in content.lines().enumerate() {
        if results.len() >= limit {
            break;
        }

        if regex.is_match(line) {
            results.push(format!(
                "{}:{}: {}",
                path.display(),
                line_num + 1,
                line.trim()
            ));
        }
    }
}
