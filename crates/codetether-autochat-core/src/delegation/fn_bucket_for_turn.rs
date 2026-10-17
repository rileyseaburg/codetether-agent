//! bucket for turn policy.
use super::*;

/// Build a Bucket for a conversation turn.
pub fn bucket_for_turn(difficulty_hint: &str, uses_tools: bool) -> Bucket {
    let difficulty = match difficulty_hint {
        "hard" => Difficulty::Hard,
        "medium" => Difficulty::Medium,
        _ => Difficulty::Easy,
    };
    Bucket {
        difficulty,
        dependency: Dependency::Isolated,
        tool_use: if uses_tools {
            ToolUse::Yes
        } else {
            ToolUse::No
        },
    }
}
