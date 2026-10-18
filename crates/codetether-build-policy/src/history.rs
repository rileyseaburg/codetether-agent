//! Search earlier requests and assistant offers after an affirmative follow-up.
use super::request_scope;
use crate::{
    assistant_offered_next_step, extract_candidate_file_paths, looks_like_build_execution_request,
    looks_like_proposed_change,
};
use codetether_context_policy::transcript::extract_text_content;
use codetether_provider_core::Message;
use std::path::Path;
pub(super) fn requires_tool(session_messages: &[Message], workspace_dir: &Path) -> bool {
    let mut skipped_latest_user = false;
    for msg in session_messages.iter().rev() {
        let msg_text = extract_text_content(&msg.content);
        if msg_text.trim().is_empty() {
            continue;
        }

        if matches!(msg.role, codetether_provider_core::Role::User) && !skipped_latest_user {
            skipped_latest_user = true;
            continue;
        }

        if matches!(msg.role, codetether_provider_core::Role::Assistant)
            && assistant_offered_next_step(&msg_text)
        {
            return true;
        }

        let request = request_scope::explicit(&msg_text);
        if (looks_like_build_execution_request(request) || looks_like_proposed_change(request))
            && !extract_candidate_file_paths(request, workspace_dir, 1).is_empty()
        {
            return true;
        }
    }

    false
}
