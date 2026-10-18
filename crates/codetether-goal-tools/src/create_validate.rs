//! Input validation for `create_goal`.

use super::create::Args;
use codetether_tool_core::ToolResult;

pub(super) fn apply(args: &mut Args) -> Option<ToolResult> {
    args.objective = args.objective.trim().to_string();
    if args.objective.is_empty() || args.objective.chars().count() > 4_000 {
        return Some(ToolResult::error(
            "objective must contain 1 to 4000 characters",
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::super::create::Args;

    #[test]
    fn trims_valid_objective() {
        let mut args = Args {
            objective: "  ship parity  ".into(),
            session_id: None,
        };
        assert!(super::apply(&mut args).is_none());
        assert_eq!(args.objective, "ship parity");
    }
}
