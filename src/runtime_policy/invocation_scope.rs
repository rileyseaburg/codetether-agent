//! Approval tuple derivation for runtime policy gates.

use serde_json::Value;
use sha2::{Digest, Sha256};

pub(super) struct InvocationScope {
    pub action: &'static str,
    pub resource: String,
}

pub(super) fn for_tool(tool_name: &str, args: &Value) -> InvocationScope {
    if tool_name == "apply_patch" {
        return patch_scope(args);
    }
    InvocationScope {
        action: "execute",
        resource: invocation_resource(tool_name, args),
    }
}

fn patch_scope(args: &Value) -> InvocationScope {
    let patch = args.get("patch").and_then(Value::as_str).unwrap_or("");
    InvocationScope {
        action: "write",
        resource: crate::tool::patch::approval_resource_from_patch(patch),
    }
}

fn invocation_resource(tool_name: &str, args: &Value) -> String {
    let mut scoped_args = args.clone();
    if let Some(map) = scoped_args.as_object_mut() {
        map.remove("approval_id");
    }
    let encoded = serde_json::to_vec(&scoped_args).unwrap_or_default();
    let digest = Sha256::digest(&encoded);
    format!("{tool_name}:{}", hex::encode(digest))
}
