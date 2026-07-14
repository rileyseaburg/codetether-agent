//! Shared validity checks for prepared transcript prefixes.

use crate::provider::Message;

use super::fingerprint;

pub(super) fn matches(messages: &[Message], count: usize, expected: u64) -> bool {
    messages
        .get(..count)
        .map(fingerprint::messages)
        .is_some_and(|value| value == expected)
}
