//! Line breaking helpers.

use super::measure::{measure_text, CHAR_WIDTH};

/// Result of breaking a text run to fit an available width.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreakToken {
    pub head: String,
    pub tail: String,
    pub forced: bool,
}

/// Break text so the returned head fits available width when possible.
pub fn break_text_to_width(text: &str, available_width: f32) -> BreakToken {
    if text.is_empty() {
        return BreakToken {
            head: String::new(),
            tail: String::new(),
            forced: false,
        };
    }
    let newline = text.find('\n');
    let before_newline = newline.map(|i| &text[..i]).unwrap_or(text);
    if measure_text(before_newline) <= available_width {
        if let Some(i) = newline {
            return BreakToken {
                head: before_newline.to_string(),
                tail: text[i + 1..].to_string(),
                forced: true,
            };
        }
        return BreakToken {
            head: text.to_string(),
            tail: String::new(),
            forced: false,
        };
    }
    let max_chars = (available_width / CHAR_WIDTH).floor().max(1.0) as usize;
    let chars: Vec<char> = before_newline.chars().collect();
    let limit = max_chars.min(chars.len());
    let mut break_at = None;
    for (i, ch) in chars.iter().enumerate().take(limit).skip(1) {
        if ch.is_whitespace() {
            break_at = Some(i);
        }
    }
    let at = break_at.unwrap_or(limit);
    let skip = usize::from(break_at.is_some());
    let head: String = chars[..at].iter().collect();
    let tail_prefix: String = chars[(at + skip).min(chars.len())..].iter().collect();
    let tail = match newline {
        Some(i) => format!("{}{}", tail_prefix, &text[i..]),
        None => tail_prefix,
    };
    BreakToken {
        head,
        tail,
        forced: false,
    }
}
