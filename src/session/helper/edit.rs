//! Shared edit normalization, stub detection and confirmation request policy.
pub use codetether_edit_policy::{
    build_pending_confirmation_apply_request, detect_stub_in_tool_input,
    normalize_tool_call_for_execution,
};
