//! Compatibility exports for shared approval gates.

pub(super) use codetether_runtime_policy::approval_gate::allowed;
pub(crate) use codetether_runtime_policy::approval_gate::approval_binding;
pub use codetether_runtime_policy::approval_gate::{
    approved_invocation, approved_or_session_command,
};
