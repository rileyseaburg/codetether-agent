//! Selector matching used to test generated selectors.

use super::super::DomNode;

pub fn matches_sel(n: &DomNode, s: &str) -> bool {
    if let Some(id) = s.strip_prefix('#') {
        return n.attr("id") == Some(id);
    }
    if s.contains(":text(\"") {
        return s.starts_with(&n.tag) && s.contains(n.text.trim());
    }
    if let Some(x) = s.strip_prefix('[').and_then(|x| x.strip_suffix(']')) {
        let mut p = x.splitn(2, "=\"");
        let k = p.next().unwrap_or("");
        let v = p.next().unwrap_or("").trim_end_matches('"');
        return n.attr(k) == Some(v);
    }
    s == n.tag
}
