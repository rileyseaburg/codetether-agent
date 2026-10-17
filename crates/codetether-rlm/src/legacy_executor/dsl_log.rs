//! Display legacy DSL results when verbose output is enabled.
use super::*;

impl RlmExecutor {
    pub(super) fn log_dsl_result(&self, execution_result: &ReplResult) {
        // Display execution results in verbose mode
        if self.verbose {
            if let Some(ref answer) = execution_result.final_answer {
                println!("[RLM] Final answer received: {}", answer);
            } else if !execution_result.stdout.is_empty() {
                let preview = truncate_with_ellipsis(&execution_result.stdout, 200);
                println!("[RLM] Execution output:\n{}", preview);
            }
        }
    }
}
