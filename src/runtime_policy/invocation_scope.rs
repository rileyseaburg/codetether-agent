//! Approval tuple derivation for runtime policy gates.

use serde_json::Value;
use sha2::{Digest, Sha256};

#[path = "invocation_scope_ralph.rs"]
mod ralph;
#[path = "invocation_scope_sanitize.rs"]
mod sanitize;
#[path = "invocation_scope_state.rs"]
mod state;

pub(crate) struct InvocationScope {
    pub(crate) action: &'static str,
    pub(crate) resource: String,
}

fn scoped_args(args: &Value) -> Value {
    sanitize::value(args)
}

pub(crate) fn for_tool(tool_name: &str, args: &Value) -> InvocationScope {
    if matches!(tool_name, "apply_patch" | "patch") {
        return patch_scope(args);
    }
    InvocationScope {
        action: "execute",
        resource: invocation_resource(tool_name, args),
    }
}

fn patch_scope(args: &Value) -> InvocationScope {
    InvocationScope {
        action: "write",
        resource: crate::tool::patch::approval_resource_from_args(args),
    }
}

fn invocation_resource(tool_name: &str, args: &Value) -> String {
    let mut scoped_args = scoped_args(args);
    ralph::bind(tool_name, &mut scoped_args);
    state::bind(tool_name, &mut scoped_args, args);
    let encoded = serde_json::to_vec(&scoped_args).unwrap_or_default();
    let digest = Sha256::digest(&encoded);
    format!("{tool_name}:{}", hex::encode(digest))
}

#[cfg(test)]
#[path = "approval_nested_tests.rs"]
mod approval_nested_tests;
#[cfg(test)]
#[path = "approval_progress_tests.rs"]
mod approval_progress_tests;
