use serde_json::json;

use crate::tool::ToolResult;

use super::candidates::whitespace_match;
use super::error::not_found;
use super::plan::MatchPlan;

impl MatchPlan {
    pub fn find(content: &str, old: &str, replace_all: bool) -> Result<Self, ToolResult> {
        let exact = content.matches(old).count();
        if exact > 0 {
            return from_target(old.to_string(), exact, replace_all, "exact");
        }
        if let Some(target) = whitespace_match(content, old) {
            return from_target(target, 1, replace_all, "whitespace");
        }
        Err(not_found(content, old))
    }
}

fn from_target(
    target: String,
    count: usize,
    replace_all: bool,
    strategy: &'static str,
) -> Result<MatchPlan, ToolResult> {
    if count > 1 && !replace_all {
        return Err(ambiguous(count));
    }
    Ok(MatchPlan {
        target,
        count,
        replace_all,
        strategy,
    })
}

fn ambiguous(count: usize) -> ToolResult {
    ToolResult::structured_error(
        "AMBIGUOUS_MATCH",
        "edit",
        &format!(
            "old_string found {count} times. Set replace_all=true to replace every occurrence, or include more context."
        ),
        None,
        Some(json!({"hint":"Set replace_all=true to replace all matches","matches_found":count})),
    )
}
