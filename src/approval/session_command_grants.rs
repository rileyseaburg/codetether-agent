//! Compatibility exports for shared session approval grants.

pub use codetether_approval_core::session_command_grants::{
    allowed, grant_for_request, remember_request,
};

#[cfg(test)]
pub(crate) use codetether_approval_core::session_command_grants::reset;
