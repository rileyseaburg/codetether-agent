//! First nonblank text-block excerpt for one dropped user message.
use super::EXCERPT_CHARS;
use codetether_provider_core::{ContentPart, Message};

pub(super) fn first_text_excerpt(msg: &Message) -> Option<String> {
    msg.content.iter().find_map(|p| match p {
        ContentPart::Text { text } if !text.trim().is_empty() => {
            let line = text.trim().lines().next().unwrap_or_default();
            let mut s: String = line.chars().take(EXCERPT_CHARS).collect();
            if line.chars().count() > EXCERPT_CHARS {
                s.push('…');
            }
            Some(s)
        }
        _ => None,
    })
}
