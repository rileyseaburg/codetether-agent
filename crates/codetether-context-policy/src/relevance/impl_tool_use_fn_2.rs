//! fn policy implementation.
use super::*;

impl ToolUse {
    /// Stable snake_case encoding — see [`Difficulty::as_str`].
    pub const fn as_str(self) -> &'static str {
        match self {
            ToolUse::No => "no",
            ToolUse::Yes => "yes",
        }
    }
}
