//! Concrete transcript fixtures for the pairing-invariant checks.

use super::super::invariant::{call, result, text};
use super::{assistant, tool};
use crate::provider::{Message, Role};

pub(super) fn interleaved() -> Vec<Message> {
    vec![
        text(Role::User, "start"),
        assistant(vec![call("call_a")]),
        tool(vec![result("call_a")]),
        assistant(vec![call("call_b")]),
        tool(vec![result("call_b")]),
    ]
}

pub(super) fn narrated() -> Vec<Message> {
    vec![
        assistant(vec![call("call_a")]),
        tool(vec![result("call_a")]),
        text(Role::Assistant, "narrating next step"),
        assistant(vec![call("call_b")]),
        tool(vec![result("call_b")]),
    ]
}

pub(super) fn stale_first() -> Vec<Message> {
    vec![
        tool(vec![result("call_stale")]),
        text(Role::User, "continue"),
        assistant(vec![call("call_new")]),
        tool(vec![result("call_new")]),
    ]
}
