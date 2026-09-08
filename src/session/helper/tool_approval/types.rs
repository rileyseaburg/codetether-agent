use serde_json::Value;

use crate::session::helper::tool_policy::ToolTuple;

pub(in crate::session::helper) enum ApprovalGate {
    Ready(Value, Vec<String>),
    Blocked(Value, ToolTuple, Vec<String>),
}

impl ApprovalGate {
    #[cfg(test)]
    pub(in crate::session::helper) fn into_parts(self) -> (Value, Option<ToolTuple>) {
        let (args, blocked, _) = self.into_checked_parts();
        (args, blocked)
    }

    pub(in crate::session::helper) fn into_checked_parts(
        self,
    ) -> (Value, Option<ToolTuple>, Vec<String>) {
        match self {
            Self::Ready(args, warnings) => (args, None, warnings),
            Self::Blocked(args, tuple, warnings) => (args, Some(tuple), warnings),
        }
    }
}
