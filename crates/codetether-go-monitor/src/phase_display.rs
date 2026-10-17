//! Append the lifecycle and final result summary to a monitor response.

use crate::{ActiveGoRun, GoRunPhase};
pub(crate) fn append(output: &mut String, run: &ActiveGoRun) {
    match &run.phase {
        GoRunPhase::Starting => {
            output.push_str("\n**Phase:** Starting (generating PRD...)\n");
        }
        GoRunPhase::Running => {
            output.push_str("\n**Phase:** Running Ralph loop\n");
        }
        GoRunPhase::Completed {
            passed,
            total,
            all_passed,
            feature_branch,
            summary,
        } => {
            output.push_str(&format!(
                "\n**Phase:** Completed {}\n\
                     **Result:** {passed}/{total} stories passed\n\
                     **Feature Branch:** `{feature_branch}`\n\n\
                     ## Summary\n\n{summary}\n",
                if *all_passed { "✅" } else { "⚠️" }
            ));
        }
        GoRunPhase::Failed { error } => {
            output.push_str(&format!("\n**Phase:** Failed ❌\n**Error:** {error}\n"));
        }
    }
}
