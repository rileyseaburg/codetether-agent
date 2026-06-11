use serde_json::Value;

use crate::session::helper::tool_policy::ToolTuple;

pub(in crate::session::helper) enum ApprovalGate {
    Ready(Value),
    Blocked(Value, ToolTuple),
}

impl ApprovalGate {
    pub(in crate::session::helper) fn into_parts(self) -> (Value, Option<ToolTuple>) {
        match self {
            Self::Ready(args) => (args, None),
            Self::Blocked(args, tuple) => (args, Some(tuple)),
        }
    }
}
