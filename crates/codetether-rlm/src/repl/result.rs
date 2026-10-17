//! Output contracts for complete scripts and individual DSL commands.

/// Result of REPL execution
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// use codetether_rlm::repl::ReplResult;
/// let result = ReplResult { stdout: String::new(), stderr: String::new(), final_answer: None };
/// assert!(result.final_answer.is_none());
/// ```
pub struct ReplResult {
    /// Combined output from evaluated commands.
    pub stdout: String,
    /// Error output reported by the execution environment.
    pub stderr: String,
    /// Final answer, when a command ended evaluation.
    pub final_answer: Option<String>,
}

/// Result of a command: printed output, final answer, or evaluation error.
///
/// # Examples
/// ```
/// use codetether_rlm::repl::DslResult;
/// let text = match DslResult::Final("done".into()) {
///     DslResult::Output(text) | DslResult::Final(text) | DslResult::Error(text) => text,
/// };
/// assert_eq!(text, "done");
/// ```
pub enum DslResult {
    /// Text produced by a command.
    Output(String),
    /// Answer that ends evaluation.
    Final(String),
    /// Text describing an evaluation error.
    #[allow(dead_code)]
    Error(String),
}
