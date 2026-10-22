//! Tool-result scanning for checkpoint state extraction.

use super::{checkpoint_text, checkpoint_walk::ScanState};
use codetether_provider_core::ContentPart;

pub(super) fn scan(parts: &[ContentPart], state: &mut ScanState) {
    for part in parts {
        if let ContentPart::ToolResult {
            tool_call_id,
            content,
        } = part
        {
            checkpoint_text::extract_browser_url(content, &mut state.browser_url);
            let status = outcome(content);
            if let Some(action) = state
                .actions
                .iter_mut()
                .rev()
                .find(|action| action.call_id == *tool_call_id && action.result_turn.is_none())
            {
                action.status = status;
                action.result_turn = Some(state.source_turn);
                if status == crate::ActionStatus::Success {
                    state.completed_actions.push(action.tool.clone());
                }
            }
            if status == crate::ActionStatus::Failed {
                state
                    .blockers
                    .push(checkpoint_text::truncate_to_line(content, 200));
            }
        }
    }
}

fn outcome(content: &str) -> crate::ActionStatus {
    use crate::ActionStatus;
    if content.starts_with("Tool call feedback\n") {
        match content.split("\n\nOutput:\n").next().and_then(|header| {
            header
                .lines()
                .find_map(|line| line.strip_prefix("- status: "))
        }) {
            Some("success") => return ActionStatus::Success,
            Some("failure") => return ActionStatus::Failed,
            _ => {}
        }
    }
    if content.trim_start().starts_with("Error:") {
        return ActionStatus::Failed;
    }
    ActionStatus::Returned
}
