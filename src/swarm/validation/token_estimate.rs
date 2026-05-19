use super::token_counts::{BASE_PROMPT_TOKENS, context_tokens, instruction_tokens};
use super::token_issues::push_token_issues;
use super::{ProviderStatus, SwarmValidator, TokenEstimate, ValidationIssue};
use crate::swarm::SubTask;

impl SwarmValidator {
    pub(super) fn estimate_token_usage(
        &self,
        subtasks: &[SubTask],
        provider_status: &ProviderStatus,
        issues: &mut Vec<ValidationIssue>,
    ) -> TokenEstimate {
        let prompt_tokens = prompt_tokens(subtasks);
        let completion_tokens = prompt_tokens / 2;
        let total_tokens = prompt_tokens + completion_tokens;
        let exceeds_limit = total_tokens > provider_status.context_window;
        push_token_issues(
            total_tokens,
            provider_status.context_window,
            exceeds_limit,
            issues,
        );
        TokenEstimate {
            prompt_tokens,
            completion_tokens,
            total_tokens,
            exceeds_limit,
            context_window: provider_status.context_window,
        }
    }
}

fn prompt_tokens(subtasks: &[SubTask]) -> usize {
    BASE_PROMPT_TOKENS + instruction_tokens(subtasks) + context_tokens(subtasks)
}
