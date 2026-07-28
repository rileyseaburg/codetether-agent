//! Inline text fragment splitting.

use super::super::fragment::{FragmentKind, InlineFragment};
use super::super::metrics::measure_text_width;

pub fn split_text_fragments(fragments: &[InlineFragment]) -> Vec<InlineFragment> {
    let mut out = Vec::new();
    for fragment in fragments {
        match &fragment.kind {
            FragmentKind::Text { content, font_size } => {
                push_text_tokens(&mut out, fragment, content, *font_size);
            }
            _ => out.push(fragment.clone()),
        }
    }
    out
}

fn push_text_tokens(out: &mut Vec<InlineFragment>, base: &InlineFragment, text: &str, size: i64) {
    for token in word_tokens(text) {
        let mut frag = base.clone();
        frag.width = measure_text_width(&token, size);
        frag.kind = FragmentKind::Text {
            content: token,
            font_size: size,
        };
        out.push(frag);
    }
}

fn word_tokens(text: &str) -> Vec<String> {
    let (mut tokens, mut buf, mut last) = (Vec::new(), String::new(), None);
    for ch in text.chars() {
        let is_space = ch.is_whitespace();
        if matches!(last, Some(space) if space != is_space) && !buf.is_empty() {
            tokens.push(std::mem::take(&mut buf));
        }
        buf.push(ch);
        last = Some(is_space);
    }
    if !buf.is_empty() {
        tokens.push(buf);
    }
    tokens
}
