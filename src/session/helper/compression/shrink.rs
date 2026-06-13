//! Payload shrinking cascade for terminal truncation.

use crate::provider::{Message, ToolDefinition};
use crate::session::helper::token::estimate_request_tokens;

use super::shrink_caps::shrink_payloads_over_cap;

/// Byte ceilings used by terminal truncation when the retained tail still
/// exceeds the provider context budget. The cascade preserves generous
/// head/tail snippets first, then gets progressively stricter.
const TERMINAL_PAYLOAD_CAPS: [usize; 8] = [
    262_144, 131_072, 65_536, 32_768, 16_384, 8_192, 4_096, 2_048,
];

/// Final emergency ceiling if the normal cascade still cannot bring the
/// prompt under budget. This should only affect pathological single-turn
/// tool dumps; the newest user instruction is already protected by the
/// oversized-message compressor before this runs.
const TERMINAL_EMERGENCY_CAP_BYTES: usize = 1_024;

/// Shrink retained payloads through the cap cascade until the request
/// fits `target_tokens`. Returns the number of content parts changed.
pub(super) fn shrink_retained_payloads_to_budget(
    messages: &mut [Message],
    system_prompt: &str,
    tools: &[ToolDefinition],
    target_tokens: usize,
) -> usize {
    if target_tokens == usize::MAX
        || estimate_request_tokens(system_prompt, messages, tools) <= target_tokens
    {
        return 0;
    }

    let mut changed_parts = 0;
    for cap in TERMINAL_PAYLOAD_CAPS {
        changed_parts += shrink_payloads_over_cap(messages, cap, false);
        if estimate_request_tokens(system_prompt, messages, tools) <= target_tokens {
            return changed_parts;
        }
    }

    changed_parts += shrink_payloads_over_cap(messages, TERMINAL_EMERGENCY_CAP_BYTES, true);
    changed_parts
}
