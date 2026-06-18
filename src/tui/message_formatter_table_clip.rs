//! Cell clipping: truncate to a column budget with an ellipsis.

use super::super::super::util::w;

/// Clip a cell to `width` columns, adding an ellipsis when truncated.
pub(super) fn clip(s: &str, width: usize) -> String {
    if w(s) <= width {
        return s.to_string();
    }
    let mut out = String::new();
    let mut used = 0usize;
    for ch in s.chars() {
        let cw = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if used + cw + 1 > width {
            break;
        }
        out.push(ch);
        used += cw;
    }
    out.push('…');
    out
}
