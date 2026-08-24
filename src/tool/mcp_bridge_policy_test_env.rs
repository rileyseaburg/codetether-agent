//! Portable sandbox availability guard for MCP bridge tests.

macro_rules! require_sandbox {
    () => {
        if crate::tool::sandbox::unavailable_reason().is_some() {
            return;
        }
    };
}

pub(super) use require_sandbox;