//! Validate parsed relay fields and choose the approval mode.
use crate::{RelayRequest, normalize_go_task_input, validate_easy_go_task};
use codetether_autochat_core::{AUTOCHAT_MAX_AGENTS, ParsedAutochatRequest};
pub(super) fn resolve(
    parsed: ParsedAutochatRequest,
    easy_go_requested: bool,
    jsonl_output: bool,
) -> anyhow::Result<RelayRequest> {
    let agent_count = parsed.agent_count;
    let parsed_task = parsed.task;
    let task = if easy_go_requested {
        validate_easy_go_task(&parsed_task)?
    } else {
        normalize_go_task_input(&parsed_task)
    };
    let require_prd = easy_go_requested || !parsed.bypass_prd;
    if require_prd && jsonl_output {
        anyhow::bail!(
            "--format jsonl does not support interactive /go approval; use /autochat --no-prd"
        );
    }

    if !(2..=AUTOCHAT_MAX_AGENTS).contains(&agent_count) {
        anyhow::bail!(
            "Invalid relay size {}. count must be between 2 and {}",
            agent_count,
            AUTOCHAT_MAX_AGENTS
        );
    }

    Ok(RelayRequest {
        agent_count,
        task,
        require_prd,
        easy_go_requested,
        explicit_count: parsed.explicit_count,
    })
}
