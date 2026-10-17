//! Evaluate script lines and collect output until a final answer.
use super::*;
impl RlmRepl {
    /// Execute analysis code (interpreted based on runtime)
    ///
    /// For Rust runtime, this uses a simple DSL:
    /// - head(n) - first n lines
    /// - tail(n) - last n lines
    /// - grep("pattern") - search for pattern
    /// - count("pattern") - count matches
    /// - slice(start, end) - slice by chars
    /// - chunks(n) - split into n chunks
    /// - FINAL({json_payload}) - return final answer payload
    ///
    /// # Arguments
    /// * `code` - DSL lines to evaluate in order.
    /// # Returns
    /// Collected output and the first final answer, when present.
    /// # Examples
    /// ```
    /// use codetether_rlm::repl::{ReplRuntime, RlmRepl};
    /// let mut repl = RlmRepl::new("first\nsecond".into(), ReplRuntime::Rust);
    /// assert_eq!(repl.execute("FINAL(\"done\")").final_answer.as_deref(), Some("done"));
    /// ```
    /// # Panics
    /// Panics for an empty context chunked with a positive count; malformed
    /// argument boundaries may also panic during expression parsing.
    pub fn execute(&mut self, code: &str) -> ReplResult {
        match self.runtime {
            ReplRuntime::Rust => self.execute_rust_dsl(code),
            ReplRuntime::Bun | ReplRuntime::Python => {
                // For external runtimes, we'd spawn processes
                // For now, fall back to DSL
                self.execute_rust_dsl(code)
            }
        }
    }

    pub(super) fn execute_rust_dsl(&mut self, code: &str) -> ReplResult {
        let mut stdout = Vec::new();
        let mut final_answer = None;

        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            // Parse and execute commands
            if let Some(result) = self.execute_dsl_line(line) {
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

        ReplResult {
            stdout: stdout.join("\n"),
            stderr: String::new(),
            final_answer,
        }
    }

    /// Execute one DSL line and convert errors to an output variant.
    ///
    /// # Arguments
    /// * `line` - A single DSL command or assignment.
    /// # Returns
    /// An output/final/error result, or None for assignments and ignored lines.
    /// # Examples
    /// ```
    /// use codetether_rlm::repl::{ReplRuntime, RlmRepl};
    /// let mut repl = RlmRepl::new("first\nsecond".into(), ReplRuntime::Rust);
    /// assert!(repl.execute_dsl_line("let x = first").is_none());
    /// ```
    /// # Panics
    /// Panics for an empty context chunked with a positive count; malformed
    /// argument boundaries may also panic during expression parsing.
    pub fn execute_dsl_line(&mut self, line: &str) -> Option<DslResult> {
        match self.try_execute_dsl_line(line) {
            Ok(res) => res,
            Err(e) => Some(DslResult::Error(e.to_string())),
        }
    }
}
