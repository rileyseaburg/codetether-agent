//! Legacy executor query sequence.
use super::*;
impl RlmExecutor {
    /// Execute code with llm_query() support
    pub(super) async fn execute_with_llm_query(&mut self, code: &str) -> Result<ReplResult> {
        let mut stdout = Vec::new();
        let mut final_answer = None;

        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            // Handle llm_query calls specially
            if line.starts_with("llm_query(") || line.contains("= llm_query(") {
                let result = self.handle_llm_query(line).await?;
                stdout.push(result);
                continue;
            }

            // Handle regular REPL commands
            if let Some(result) = self.repl.execute_dsl_line(line) {
                match result {
                    DslResult::Output(s) => stdout.push(s),
                    DslResult::Final(s) => {
                        final_answer = Some(s);
                        break;
                    }
                    DslResult::Error(s) => stdout.push(format!("Error: {}", s)),
                }
            }
        }

        Ok(ReplResult {
            stdout: stdout.join("\n"),
            stderr: String::new(),
            final_answer,
        })
    }
}
