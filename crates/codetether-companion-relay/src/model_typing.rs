//! One-shot handoff from a fresh owner typing request to the device reply queue.
use crate::{relay::now, runtime::Runtime, typing_proposal};
use serde_json::json;

/// Accept direct typing instructions, never authorization from screen contents.
pub(crate) fn requests_typing(prompt: &str) -> bool {
    let lower = prompt.trim().to_lowercase();
    let mut words = lower.split_whitespace().peekable();
    if words.peek() == Some(&"please") {
        words.next();
    }
    if matches!(words.peek(), Some(&"can" | &"could" | &"would" | &"will")) {
        words.next();
        if words.next() != Some("you") {
            return false;
        }
    }
    if words.peek() == Some(&"please") {
        words.next();
    }
    matches!(words.next(), Some("type" | "enter" | "write" | "fill")) && words.next().is_some()
}
/// Called once after success, under the live session lock; never on snapshots.
pub(crate) fn finish(runtime: &mut Runtime, authorized: bool) {
    if !authorized {
        return;
    }
    let Some((prose, proposal)) = typing_proposal::parse(&runtime.text) else {
        runtime.text = "No typing queued: the model did not return valid keyboard text. Ask again with the intended field visible.".into();
        return;
    };
    let queued = runtime
        .queue_reply(&json!({ "text": proposal.text }), true, now())
        .is_ok();
    let notice = if queued {
        "Typing queued for Windows, not confirmed as inserted. No automatic retry."
    } else {
        "Typing was not queued. Check Windows before making a new request."
    };
    runtime.text = format!(
        "{prose}\n\n{notice}\n\nTarget: {}\n\n```text\n{}\n```",
        proposal.target, proposal.text
    );
}
