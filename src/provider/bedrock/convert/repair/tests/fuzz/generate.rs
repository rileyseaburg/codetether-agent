//! Deterministic pseudo-random transcript generator (no external crates).

use super::super::invariant::{call, result, text};
use crate::provider::{ContentPart, Message, Role};

fn assistant(parts: Vec<ContentPart>) -> Message {
    Message {
        role: Role::Assistant,
        content: parts,
    }
}

fn tool(parts: Vec<ContentPart>) -> Message {
    Message {
        role: Role::Tool,
        content: parts,
    }
}

pub(super) fn transcript(seed: u64) -> Vec<Message> {
    let mut state = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    let mut next = move || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (state >> 33) as usize
    };
    let mut messages = Vec::new();
    let mut calls = 0usize;
    for _ in 0..12 {
        match next() % 6 {
            0 => messages.push(text(Role::User, "u")),
            1 => messages.push(text(Role::Assistant, "a")),
            2 => messages.push(text(Role::System, "s")),
            3 => {
                calls += 1;
                messages.push(assistant(vec![call(&format!("call_{calls}"))]));
            }
            4 if calls > 0 => messages.push(tool(vec![result(&format!("call_{calls}"))])),
            _ => messages.push(tool(vec![result("call_orphan")])),
        }
    }
    messages
}
