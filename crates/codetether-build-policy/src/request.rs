//! Resolve actionable build requests from current and previous messages.
use super::request_scope;
use crate::{
    extract_candidate_file_paths, is_affirmative_build_followup, looks_like_build_execution_request,
};
use codetether_context_policy::transcript::latest_user_text;
use codetether_provider_core::Message;
use std::path::Path;

/// Apply the established build request requires tool rule.
/// # Arguments
/// Receives the message, tool or workspace inputs named in the signature.
/// # Returns
/// The decision or matching paths without executing tools.
/// # Examples
/// ```
/// assert!(!codetether_build_policy::build_request_requires_tool(&[], std::path::Path::new(".")));
/// ```
pub fn build_request_requires_tool(session_messages: &[Message], workspace_dir: &Path) -> bool {
    let Some(text) = latest_user_text(session_messages) else {
        return false;
    };

    let request = request_scope::explicit(&text);
    if looks_like_build_execution_request(request)
        && !extract_candidate_file_paths(request, workspace_dir, 1).is_empty()
    {
        return true;
    }

    if !is_affirmative_build_followup(&text) {
        return false;
    }

    super::history::requires_tool(session_messages, workspace_dir)
}
