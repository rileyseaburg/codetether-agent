//! Explicit authority for Windows' unsandboxed PowerShell transport.

use crate::tool::ToolResult;
use serde_json::Value;

pub(super) fn blocked(args: &Value) -> Option<ToolResult> {
    if !cfg!(windows) {
        return None;
    }
    if !signed_network(args) {
        return Some(ToolResult::structured_error(
            "NETWORK_ACCESS_DISABLED",
            "computer_use",
            "Windows computer_use requires signed network authority because PowerShell is unsandboxed.",
            None,
            None,
        ));
    }
    let allowed = authorized(
        crate::tool::sandbox::direct_fallback_env_allowed(),
        crate::runtime_policy::approved_receipt("computer_use", args),
        true,
    );
    (!allowed).then(|| {
        ToolResult::structured_error(
            "UNSAFE_FALLBACK_REQUIRED",
            "computer_use",
            "Windows computer_use requires an exact approval and the explicit unsafe fallback setting.",
            None,
            None,
        )
    })
}

fn authorized(unsafe_fallback: bool, exact_approval: bool, network: bool) -> bool {
    unsafe_fallback && exact_approval && network
}

fn signed_network(args: &Value) -> bool {
    crate::tool::network_access::trusted_value(args) == Some(true)
}

#[cfg(test)]
#[path = "unsafe_process_policy_tests.rs"]
mod tests;
