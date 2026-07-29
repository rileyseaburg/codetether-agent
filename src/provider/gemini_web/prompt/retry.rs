//! Size-bounded corrective suffix for one invalid-protocol retry.

use crate::provider::util::truncate_bytes_safe;

pub(super) const RESERVE_BYTES: usize = 768;
const PREFIX: &str = "System: Previous response rejected by tool protocol validation: ";
// The reminder deliberately avoids literal protocol tags: `render` sanitizes the
// error text so a hostile message cannot inject markup, and emitting real tags
// here would reintroduce that hazard. Point back at the rules block instead.
const SUFFIX: &str = concat!(
    ". Regenerate the response, obey the catalog, and emit only complete ",
    "independent calls. Use the exact raw tool-call block format shown in the ",
    "gemini_web_tool_protocol rules, with a JSON object carrying `name` and ",
    "`arguments`. Describing a call in prose does not invoke it."
);

pub(super) fn render(original: &str, error: &str) -> String {
    let safe = error.replace('<', "[").replace('>', "]");
    let room = RESERVE_BYTES.saturating_sub(PREFIX.len() + SUFFIX.len() + 1);
    let reason = truncate_bytes_safe(&safe, room);
    format!("{original}\n{PREFIX}{reason}{SUFFIX}")
}

#[cfg(test)]
#[path = "retry_tests.rs"]
mod tests;
