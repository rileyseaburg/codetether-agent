//! analyze for PRD construction and validation.
use super::*;
impl PrdTool {
    pub(super) async fn analyze(&self, p: Params) -> Result<ToolResult> {
        let task = p.task_description.unwrap_or_default();
        if task.is_empty() {
            return Ok(ToolResult::structured_error(
                "missing_field",
                self.id(),
                "task_description is required for analyze",
                Some(vec!["task_description"]),
                Some(json!({"action":"analyze","task_description":"..."})),
            ));
        }

        let mut questions = String::new();
        questions.push_str("# Task Analysis\n\n");
        questions.push_str("## Task Description\n");
        questions.push_str(&task);
        questions.push_str("\n\n");
        questions.push_str("## Questions to Answer\n\n");
        questions.push_str("1. **Project Name**\n");
        questions.push_str("2. **Feature Name**\n");
        questions.push_str(
                    "3. **User Stories**: break down the task into discrete, independently verifiable stories.\n",
                );
        questions.push_str("   For each story:\n");
        questions.push_str("   - ID (e.g., US-001)\n");
        questions.push_str("   - Title\n");
        questions.push_str("   - Description\n");
        questions.push_str("   - Acceptance Criteria\n");
        questions.push_str(
            "   - Verification Steps (machine-verifiable evidence; BDD/TDD/E2E/artifacts/URLs)\n",
        );
        questions.push_str("   - Priority (1=highest)\n");
        questions.push_str("   - Dependencies\n");
        questions.push_str("   - Complexity (1-5)\n\n");
        questions.push_str("4. **Quality Checks** (repo-level gates; optional)\n\n");

        questions.push_str("### Example story verification steps\n\n");
        questions.push_str("```json\n");
        questions.push_str(verification_example::VERIFICATION_EXAMPLE_JSON);
        questions.push_str("\n```\n");

        Ok(ToolResult::success(questions))
    }
}
