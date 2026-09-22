//! Usage mapping for Copilot responses.

use super::types::CopilotUsage;
use crate::provider::Usage;

pub(super) fn from_copilot(usage: Option<&CopilotUsage>) -> Usage {
    usage.map_or_else(Usage::default, |u| Usage {
        prompt_tokens: u.prompt_tokens,
        completion_tokens: u.completion_tokens,
        total_tokens: u.total_tokens,
        ..Default::default()
    })
}
