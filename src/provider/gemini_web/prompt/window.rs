//! Turn-safe request bounding for the undocumented Gemini Web endpoint.

use crate::provider::{Message, Role};

mod oversized;
mod turns;

pub(super) const MAX_BYTES: usize = 256 * 1024;
const OMITTED: &str = "System: [Earlier conversation omitted to fit Gemini Web request limit.]";

pub(super) fn fit(messages: &[Message], rendered: Vec<String>, limit: usize) -> Vec<String> {
    if joined_len(&rendered) <= limit {
        return rendered;
    }
    let prefix = messages
        .iter()
        .take_while(|message| matches!(message.role, Role::System | Role::Developer))
        .count();
    let starts = turns::starts(messages);
    let fixed = joined_len(&rendered[..prefix]) + OMITTED.len() + 2;
    if fixed > limit {
        return oversized::fit(&rendered, prefix, &starts, limit, OMITTED);
    }
    let mut available = limit.saturating_sub(fixed);
    let mut selected = rendered.len();
    for range in starts.windows(2).rev() {
        let size = joined_len(&rendered[range[0]..range[1]]) + 1;
        if size > available {
            break;
        }
        available -= size;
        selected = range[0];
    }
    let tail = if selected < rendered.len() {
        rendered[selected..].to_vec()
    } else {
        turns::latest_user(&rendered, &starts, available)
    };
    let mut output = rendered[..prefix].to_vec();
    output.push(OMITTED.to_string());
    output.extend(tail);
    output
}

fn joined_len(items: &[String]) -> usize {
    items.iter().map(String::len).sum::<usize>() + items.len().saturating_sub(1)
}
