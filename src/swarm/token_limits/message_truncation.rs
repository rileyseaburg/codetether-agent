use super::constants::{RESPONSE_RESERVE_TOKENS, TRUNCATION_THRESHOLD};
use super::estimation::estimate_total_tokens;
use super::summary::summarize_removed_messages;
use super::truncation::truncate_large_tool_results;
use crate::provider::{ContentPart, Message, Role};

pub fn truncate_messages_to_fit(messages: &mut Vec<Message>, context_limit: usize) -> usize {
    let target_tokens =
        ((context_limit as f64) * TRUNCATION_THRESHOLD) as usize - RESPONSE_RESERVE_TOKENS;
    if estimate_total_tokens(messages) <= target_tokens {
        return 0;
    }
    truncate_large_tool_results(messages, 2000);
    if estimate_total_tokens(messages) <= target_tokens || messages.len() <= 6 {
        return 0;
    }
    remove_middle_messages(messages)
}

fn remove_middle_messages(messages: &mut Vec<Message>) -> usize {
    let keep_start = 2;
    let keep_end = 4;
    let removable_count = messages.len().saturating_sub(keep_start + keep_end);
    if removable_count == 0 {
        return 0;
    }
    let removed: Vec<_> = messages
        .drain(keep_start..keep_start + removable_count)
        .collect();
    let summary = summarize_removed_messages(&removed);
    messages.insert(keep_start, summary_message(removed.len(), summary));
    removed.len()
}

fn summary_message(removed: usize, summary: String) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: format!(
                "[Context truncated: {removed} earlier messages removed to fit context window]\n{summary}"
            ),
        }],
    }
}
