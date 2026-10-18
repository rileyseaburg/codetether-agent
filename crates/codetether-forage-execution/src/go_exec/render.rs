//! Render successful and partially successful Go-engine outcomes.
use anyhow::Result;
use codetether_go_ralph::GoRalphResult;
pub(super) fn result(result: GoRalphResult) -> Result<String> {
    if result.all_passed {
        Ok(format!(
            "go execution completed - all {}/{} stories passed (iterations: {}/{}, branch: {})",
            result.passed,
            result.total,
            result.iterations,
            result.max_iterations,
            result.feature_branch
        ))
    } else {
        Ok(format!(
            "go execution completed - {}/{} stories passed (iterations: {}/{}, branch: {}, status: {:?})",
            result.passed,
            result.total,
            result.iterations,
            result.max_iterations,
            result.feature_branch,
            result.status
        ))
    }
}
