//! Column alignment parsed from the GFM separator row.

use super::super::util::w;

/// Per-column text alignment.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Align {
    Left,
    Center,
    Right,
}

/// Parse alignment markers from a separator row, e.g. `:---`, `:--:`, `---:`.
pub(crate) fn parse_aligns(separator_row: &str) -> Vec<Align> {
    super::super::parse::split_cells(separator_row)
        .iter()
        .map(|c| {
            let t = c.trim();
            let left = t.starts_with(':');
            let right = t.ends_with(':');
            match (left, right) {
                (true, true) => Align::Center,
                (false, true) => Align::Right,
                _ => Align::Left,
            }
        })
        .collect()
}

/// Pad `s` to `width` columns according to `a`.
pub(crate) fn align(s: &str, width: usize, a: Align) -> String {
    let used = w(s);
    if used >= width {
        return s.to_string();
    }
    let slack = width - used;
    match a {
        Align::Left => format!("{s}{}", " ".repeat(slack)),
        Align::Right => format!("{}{s}", " ".repeat(slack)),
        Align::Center => {
            let l = slack / 2;
            format!("{}{s}{}", " ".repeat(l), " ".repeat(slack - l))
        }
    }
}
