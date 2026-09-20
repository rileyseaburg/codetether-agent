//! Split a flowchart statement into `(left, arrow, right)` around an arrow.

/// Arrow tokens recognised in flowchart statements, longest first so that
/// `-.->` is not mistaken for `-`.
const ARROWS: [&str; 6] = ["-.->", "==>", "-->", "---", "->", "--"];

/// Locate the first arrow in `line` and return the surrounding text.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::arrow::split_arrow;
///
/// let (l, a, r) = split_arrow("A --> B").unwrap();
/// assert_eq!((l.trim(), a, r.trim()), ("A", "-->", "B"));
/// assert!(split_arrow("A B").is_none());
/// ```
pub fn split_arrow(line: &str) -> Option<(&str, &'static str, &str)> {
    let mut best: Option<(usize, &'static str)> = None;
    for arrow in ARROWS {
        if let Some(idx) = line.find(arrow)
            && best.is_none_or(|(b, _)| idx < b)
        {
            best = Some((idx, arrow));
        }
    }
    let (idx, arrow) = best?;
    Some((&line[..idx], arrow, &line[idx + arrow.len()..]))
}

/// Strip a leading `|label|` from the right-hand side of an arrow.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::parse::arrow::take_label;
///
/// let (label, rest) = take_label("|yes| B");
/// assert_eq!(label.as_deref(), Some("yes"));
/// assert_eq!(rest.trim(), "B");
/// assert_eq!(take_label("B").0, None);
/// ```
pub fn take_label(rest: &str) -> (Option<String>, &str) {
    let trimmed = rest.trim_start();
    if let Some(body) = trimmed.strip_prefix('|')
        && let Some(end) = body.find('|')
    {
        let label = body[..end].trim().trim_matches('"').to_string();
        return (Some(label), &body[end + 1..]);
    }
    (None, rest)
}
