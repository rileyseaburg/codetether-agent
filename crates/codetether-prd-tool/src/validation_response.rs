//! validation response for PRD generation and validation.
use super::*;
pub(super) fn render(prd: &Prd) -> Result<ToolResult> {
    let validation = validate_prd(prd);

    if validation.is_valid {
        Ok(ToolResult::success(format!(
            "# PRD Validation: PASSED\n\nProject: {}\nFeature: {}\nStories: {}\nExecution stages: {}\n\n{}",
            prd.project,
            prd.feature,
            prd.user_stories.len(),
            prd.stages().len(),
            validation.summary()
        )))
    } else {
        Ok(ToolResult::error(format!(
            "# PRD Validation: FAILED\n\n{}",
            validation.summary()
        )))
    }
}
