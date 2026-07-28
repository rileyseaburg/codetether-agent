//! Active stylesheet projection for bounded `@media` support.

use crate::browser_agent::MediaEmulation;

use super::conditions;

pub(crate) fn active_css(source: &str, width: i64, media: MediaEmulation) -> String {
    let mut out = String::new();
    let mut rest = source;
    loop {
        let lower = rest.to_ascii_lowercase();
        let Some(at) = lower.find("@media") else {
            out.push_str(rest);
            return out;
        };
        out.push_str(&rest[..at]);
        let after = &rest[at + "@media".len()..];
        let Some(open_rel) = after.find('{') else {
            out.push_str(&rest[at..]);
            return out;
        };
        let open = at + "@media".len() + open_rel;
        let Some(close) = matching_brace(rest, open) else {
            out.push_str(&rest[at..]);
            return out;
        };
        let query = after[..open_rel].trim();
        if conditions::matches(query, width, media) {
            out.push_str(&active_css(&rest[open + 1..close], width, media));
        }
        rest = &rest[close + 1..];
    }
}

fn matching_brace(source: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, byte) in source.bytes().enumerate().skip(open) {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}
