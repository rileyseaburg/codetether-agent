//! Input validation for `create_goal`.

use super::create::Args;
use crate::tool::ToolResult;

pub(super) fn apply(args: &mut Args) -> Option<ToolResult> {
    args.objective = args.objective.trim().to_string();
    if args.objective.is_empty() || args.objective.chars().count() > 4_000 {
        return Some(ToolResult::error(
            "objective must contain 1 to 4000 characters",
        ));
    }
    args.token_budget
        .is_some_and(|budget| budget <= 0)
        .then(|| ToolResult::error("token_budget must be positive"))
}

#[cfg(test)]
mod tests {
    use super::super::create::Args;

    #[test]
    fn trims_valid_objective() {
        let mut args = Args {
            objective: "  ship parity  ".into(),
            token_budget: Some(10),
            session_id: None,
        };
        assert!(super::apply(&mut args).is_none());
        assert_eq!(args.objective, "ship parity");
    }
}
