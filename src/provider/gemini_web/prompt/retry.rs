//! Size-bounded corrective suffix for one invalid-protocol retry.

use crate::provider::util::truncate_bytes_safe;

pub(super) const RESERVE_BYTES: usize = 512;
const PREFIX: &str = "System: Previous response rejected by tool protocol validation: ";
const SUFFIX: &str =
    ". Regenerate the response, obey the catalog, and emit only complete independent calls.";

pub(super) fn render(original: &str, error: &str) -> String {
    let safe = error.replace('<', "[").replace('>', "]");
    let room = RESERVE_BYTES.saturating_sub(PREFIX.len() + SUFFIX.len() + 1);
    let reason = truncate_bytes_safe(&safe, room);
    format!("{original}\n{PREFIX}{reason}{SUFFIX}")
}

#[cfg(test)]
#[path = "retry_tests.rs"]
mod tests;
