//! Compatibility exports for shared session approval grants.

pub use codetether_approval_core::session_command_grants::{
    allowed, grant_for_request, remember_request,
};

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub use codetether_approval_core::session_command_grants::reset;
