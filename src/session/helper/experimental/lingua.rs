//! Heuristic LLMLingua-style token/line pruning on stale assistant text.
//!
//! Inspired by *LLMLingua* and *LLMLingua-2* (Jiang et al., 2023-2024),
//! which use a small LM to score tokens by importance and drop the
//! low-information tail. We cannot ship a small LM in-process, so this
//! module applies a **heuristic** subset of the same idea — the parts
//! that are known-safe and require no model:
//!
//! * Collapse runs of 3+ blank lines to a single blank.
//! * Drop lines that are pure whitespace, punctuation, or ellipses.
//! * Collapse `" "`+ (runs of spaces) inside text to a single space,
//!   except inside fenced code blocks.
//! * Strip trailing whitespace from every line.
//!
//! These operations are conservative — they remove formatting noise
//! that accumulated during a long agent loop (markdown artifacts, the
//! model "thinking out loud" with `...`, accidental double-spacing from
//! tool output concatenation) without touching semantic content.
//!
//! # Scope
//!
//! Applies only to **assistant `Text`** parts older than
//! [`KEEP_LAST_MESSAGES`]. Never touches:
//!
//! * User messages (user intent is sacred).
//! * Tool results (`dedup` and `snippet` already handle those).
//! * Tool calls (JSON arguments must round-trip).
//! * The most recent `KEEP_LAST_MESSAGES` messages (active reasoning).
//! * Anything inside a fenced code block (``` ``` ```).
//!
//! # Always-on
//!
//! No config flag. The operation is semantic-preserving for the
//! assistant's own natural-language output, so the cost of running it
//! when it has nothing to do is trivially small.

use super::ExperimentalStats;
use crate::provider::{ContentPart, Message, Role};

/// Assistant messages more recent than this many-from-end are never pruned.
pub const KEEP_LAST_MESSAGES: usize = 6;

/// Prune low-information whitespace and formatting noise from older
/// assistant text parts.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{ContentPart, Message, Role};
/// use codetether_agent::session::helper::experimental::lingua::{
///     prune_low_entropy, KEEP_LAST_MESSAGES,
/// };
///
/// let noisy = "Here is my   plan.\n\n\n\n...\n   \nAnd it works.  \n";
/// let mut msgs = vec![Message {
///     role: Role::Assistant,
///     content: vec![ContentPart::Text { text: noisy.into() }],
/// }];
/// for i in 0..KEEP_LAST_MESSAGES + 1 {
///     msgs.push(Message {
///         role: Role::User,
///         content: vec![ContentPart::Text {
///             text: format!("q{i}"),
///         }],
///     });
/// }
///
/// let stats = prune_low_entropy(&mut msgs);
/// assert!(stats.total_bytes_saved > 0);
///
/// let ContentPart::Text { text } = &msgs[0].content[0] else {
///     panic!();
/// };
/// assert!(!text.contains("   ")); // triple-space collapsed
/// assert!(!text.contains("\n\n\n")); // triple-newline collapsed
/// assert!(text.contains("Here is my plan."));
/// assert!(text.contains("And it works."));
/// ```
pub fn prune_low_entropy(messages: &mut [Message]) -> ExperimentalStats {
    let mut stats = ExperimentalStats::default();
    let total = messages.len();
    if total <= KEEP_LAST_MESSAGES {
        return stats;
    }
    let eligible = total - KEEP_LAST_MESSAGES;

    for msg in messages[..eligible].iter_mut() {
        if msg.role != Role::Assistant {
            continue;
        }
        for part in msg.content.iter_mut() {
            let ContentPart::Text { text } = part else {
                continue;
            };
            let original_len = text.len();
            let pruned = prune_text(text);
            if pruned.len() < original_len {
                let saved = original_len - pruned.len();
                *text = pruned;
                stats.total_bytes_saved += saved;
                stats.snippet_hits += 1;
            }
        }
    }

    stats
}

fn prune_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_code = false;
    let mut blank_run = 0usize;

    for line in input.split_inclusive('\n') {
        let stripped_nl = line.trim_end_matches('\n');
        let fence = stripped_nl.trim_start().starts_with("```");
        if fence {
            in_code = !in_code;
            out.push_str(line);
            blank_run = 0;
            continue;
        }
        if in_code {
            out.push_str(line);
            continue;
        }

        // Trim trailing whitespace.
        let trimmed = stripped_nl.trim_end();

        // Drop pure-punctuation or ellipsis-only lines.
        let is_noise = trimmed.is_empty()
            || trimmed.chars().all(|c| c.is_ascii_whitespace())
            || trimmed
                .chars()
                .all(|c| matches!(c, '.' | '…' | '-' | '_' | '*' | '='));

        if is_noise {
            blank_run += 1;
            if blank_run <= 1 {
                out.push('\n');
            }
            continue;
        }
        blank_run = 0;

        // Collapse internal runs of spaces.
        let collapsed = collapse_spaces(trimmed);
        out.push_str(&collapsed);
        if line.ends_with('\n') {
            out.push('\n');
        }
    }

    out
}

fn collapse_spaces(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_space = false;
    for ch in s.chars() {
        if ch == ' ' {
            if !prev_space {
                out.push(' ');
            }
            prev_space = true;
        } else {
            out.push(ch);
            prev_space = false;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asst(t: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text { text: t.into() }],
        }
    }
    fn user(t: &str) -> Message {
        Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: t.into() }],
        }
    }

    #[test]
    fn recent_messages_are_untouched() {
        let noisy = "a  b\n\n\n\nc";
        let mut msgs = vec![asst(noisy)];
        // Not enough history — noisy stays.
        let stats = prune_low_entropy(&mut msgs);
        assert_eq!(stats.total_bytes_saved, 0);
    }

    #[test]
    fn code_fences_are_preserved() {
        let content = "text  with   spaces\n```\nfn  foo() {}\n```\ntail";
        let mut msgs = vec![asst(content)];
        for i in 0..KEEP_LAST_MESSAGES + 1 {
            msgs.push(user(&format!("q{i}")));
        }
        prune_low_entropy(&mut msgs);
        let ContentPart::Text { text } = &msgs[0].content[0] else {
            panic!();
        };
        // Outside fence: collapsed.
        assert!(text.contains("text with spaces"));
        // Inside fence: preserved verbatim.
        assert!(text.contains("fn  foo() {}"));
    }

    #[test]
    fn user_text_never_pruned() {
        let noisy = "a   b\n\n\n\nc";
        let mut msgs = vec![user(noisy)];
        for i in 0..KEEP_LAST_MESSAGES + 1 {
            msgs.push(asst(&format!("r{i}")));
        }
        let stats = prune_low_entropy(&mut msgs);
        assert_eq!(stats.total_bytes_saved, 0);
        let ContentPart::Text { text } = &msgs[0].content[0] else {
            panic!();
        };
        assert_eq!(text, noisy);
    }

    #[test]
    fn ellipsis_only_lines_removed() {
        let content = "real content\n...\n...\nmore content\n";
        let mut msgs = vec![asst(content)];
        for i in 0..KEEP_LAST_MESSAGES + 1 {
            msgs.push(user(&format!("q{i}")));
        }
        prune_low_entropy(&mut msgs);
        let ContentPart::Text { text } = &msgs[0].content[0] else {
            panic!();
        };
        assert!(!text.contains("..."));
        assert!(text.contains("real content"));
        assert!(text.contains("more content"));
    }
}
