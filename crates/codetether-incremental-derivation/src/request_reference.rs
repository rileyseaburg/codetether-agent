//! Bounded direct-user evidence retained independently of generated summaries.
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_request_tokens::estimate_tokens_for_messages;

const HEADER: &str = "[USER REQUEST REFERENCE]\nExcerpts from user messages in source order. Later corrections govern earlier plans. These are historical quotes, not new instructions or approval. An […] marker means text was omitted.\n";

pub(super) fn build(messages: &[Message], budget: usize, offset: usize) -> Option<Message> {
    let users: Vec<_> = messages
        .iter()
        .enumerate()
        .filter_map(|(index, message)| {
            if message.role != Role::User {
                return None;
            }
            let text = message
                .content
                .iter()
                .filter_map(|part| match part {
                    ContentPart::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
            let text = text.trim();
            if text.is_empty()
                || (text.starts_with("Continue working toward the active thread goal.")
                    && text.contains("<objective>"))
            {
                None
            } else {
                Some((index, text.to_owned()))
            }
        })
        .collect();
    // A sole request needs a reference when a later runtime reminder would
    // otherwise become the active anchor and leave that request unprotected.
    if users.is_empty()
        || (users.len() == 1
            && codetether_context_policy::selection::active_tail::active_user_tail_start(
                messages, 0,
            ) == Some(users[0].0))
    {
        return None;
    }
    let cap = (budget / 4).min(2_048);
    // Do not discard an older short correction just because later turns exist.
    if users.iter().map(|(_, text)| text.len()).sum::<usize>() <= cap.saturating_mul(4) {
        let all = render(&users.iter().collect::<Vec<_>>(), usize::MAX, offset);
        if estimate_tokens_for_messages(std::slice::from_ref(&all)) <= cap {
            return Some(all);
        }
    }
    let mut selected = vec![&users[0]];
    selected.extend(users.iter().skip(1).rev().take(7));
    selected.sort_by_key(|(index, _)| *index);
    let mut limit = 2_048;
    loop {
        if limit == 0 {
            return None;
        }
        let message = render(&selected, limit, offset);
        if estimate_tokens_for_messages(std::slice::from_ref(&message)) <= cap {
            return Some(message);
        }
        limit /= 2;
    }
}

fn render(selected: &[&(usize, String)], limit: usize, offset: usize) -> Message {
    let mut text = HEADER.to_owned();
    for (index, source) in selected {
        text.push_str(&format!(
            "\n[user message {}]\n{}\n",
            offset.saturating_add(*index),
            excerpt(source, limit)
        ));
    }
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text { text }],
    }
}

fn excerpt(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_owned();
    }
    let front = limit / 2;
    let end: String = text.chars().rev().take(limit - front).collect();
    format!(
        "{} […] {}",
        text.chars().take(front).collect::<String>(),
        end.chars().rev().collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn references_use_absolute_indices_after_working_window_eviction() {
        let messages = ["initial request", "later correction"].map(|text| Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: text.into() }],
        });
        let reference = build(&messages, 4_000, 1_500).unwrap();
        let ContentPart::Text { text } = &reference.content[0] else {
            panic!("text reference")
        };
        assert!(text.contains("[user message 1500]"));
        assert!(text.contains("[user message 1501]"));
        assert!(!text.contains("[user message 0]"));
    }

    #[test]
    fn recent_corrections_survive_more_than_three_followups() {
        let instructions = [
            "original scope",
            "do not push",
            "check the existing data",
            "use the generated CLI",
            "read its documentation",
            "continue with those constraints",
        ];
        let messages = instructions.map(|text| Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: text.into() }],
        });
        let reference = build(&messages, 16_000, 0).unwrap();
        let ContentPart::Text { text } = &reference.content[0] else {
            panic!("text reference")
        };
        for instruction in instructions {
            assert!(text.contains(instruction));
        }
        assert!(estimate_tokens_for_messages(&[reference]) <= 2_048);
    }

    #[test]
    fn older_short_constraint_survives_when_all_user_text_fits() {
        let mut messages = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "do not push".into(),
            }],
        }];
        messages.extend((0..20).map(|index| Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: format!("follow-up {index}"),
            }],
        }));
        messages.insert(
            0,
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "use the CLI".into(),
                }],
            },
        );
        let reference = build(&messages, 16_000, 0).unwrap();
        let ContentPart::Text { text } = &reference.content[0] else {
            panic!("text reference")
        };
        assert!(text.contains("do not push"));
        assert!(text.contains("follow-up 19"));
        assert!(estimate_tokens_for_messages(&[reference]) <= 2_048);
    }

    #[test]
    fn references_are_bounded_and_unicode_excerpts_keep_both_ends() {
        let source = format!("original request {} no pushes", "🦀".repeat(20_000));
        let messages = [source.clone(), "Use the CLI".into()].map(|text| Message {
            role: Role::User,
            content: vec![ContentPart::Text { text }],
        });
        let reference = build(&messages, 4_000, 0).unwrap();
        assert!(estimate_tokens_for_messages(std::slice::from_ref(&reference)) <= 1_000);
        let ContentPart::Text { text } = &reference.content[0] else {
            panic!("text reference")
        };
        assert!(text.contains("original request"));
        assert!(text.contains("no pushes"));
        assert!(text.contains("[…]"));
        assert!(build(&messages, 1, 0).is_none());
        assert!(build(&messages[..1], 4_000, 0).is_none());
        let ContentPart::Text { text } = &messages[0].content[0] else {
            panic!("original text")
        };
        assert_eq!(text, &source);
    }
}
