//! bucket policy.
use super::*;

pub(super) fn bucket() -> Bucket {
    Bucket {
        difficulty: Difficulty::Easy,
        dependency: Dependency::Isolated,
        tool_use: ToolUse::No,
    }
}
