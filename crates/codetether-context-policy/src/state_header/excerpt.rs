//! First nonempty line excerpt for a pinned message.
use super::*;

pub(super) fn first_text_excerpt(msg: &Message) -> Option<String> {
    msg.content.iter().find_map(|part| match part {
        ContentPart::Text { text } if !text.trim().is_empty() => {
            let line = text.trim().lines().next().unwrap_or_default();
            let mut out: String = line.chars().take(EXCERPT_CHARS).collect();
            if line.chars().count() > EXCERPT_CHARS {
                out.push('…');
            }
            Some(out)
        }
        _ => None,
    })
}
