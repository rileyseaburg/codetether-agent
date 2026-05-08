//! Structured token-exhaustion reporting for sub-agents.
//!
//! When a long-running sub-agent hits context/token limits it must
//! return a structured failure that carries partial progress and
//! recovery instructions so the parent can resume or split the task.

use serde::{Deserialize, Serialize};

/// Structured report produced when a sub-agent exhausts its token budget.
///
/// Contains enough context for a parent agent to understand what
/// happened, what was accomplished, and how to continue.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::swarm::token_exhaustion::TokenExhaustionReport;
///
/// let report = TokenExhaustionReport {
///     subtask_id: "st-001".into(),
///     tokens_used: 256_000,
///     tokens_limit: 256_000,
///     steps_completed: 42,
///     changed_files: vec!["src/main.rs".into()],
///     last_tool_calls: vec!["edit".into(), "bash".into()],
///     errors_encountered: vec![],
///     partial_output: "Implemented 3 of 5 items".into(),
///     recovery_hint: "Resume from item 4".into(),
/// };
/// assert!(!report.changed_files.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenExhaustionReport {
    /// The subtask that hit the limit.
    pub subtask_id: String,
    /// Estimated tokens consumed.
    pub tokens_used: usize,
    /// Token budget that was exceeded.
    pub tokens_limit: usize,
    /// Steps completed before exhaustion.
    pub steps_completed: usize,
    /// Files that were modified (best-effort).
    pub changed_files: Vec<String>,
    /// Last few tool call names for forensic context.
    pub last_tool_calls: Vec<String>,
    /// Errors encountered during the run.
    pub errors_encountered: Vec<String>,
    /// Whatever partial text output the agent produced.
    pub partial_output: String,
    /// Human-readable instruction for the parent/resumer.
    pub recovery_hint: String,
}

/// Recovery advice generated from a token-exhaustion event.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::swarm::token_exhaustion::{TokenExhaustionReport, RecoveryAdvice};
///
/// let report = TokenExhaustionReport {
///     subtask_id: "st-001".into(),
///     tokens_used: 300_000,
///     tokens_limit: 256_000,
///     steps_completed: 20,
///     changed_files: vec!["src/lib.rs".into()],
///     last_tool_calls: vec!["read".into()],
///     errors_encountered: vec![],
///     partial_output: "Partially done".into(),
///     recovery_hint: "Continue from step 21".into(),
/// };
/// let advice = RecoveryAdvice::from_report(&report);
/// assert!(advice.can_resume);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryAdvice {
    /// Whether the task is resumable (had partial progress).
    pub can_resume: bool,
    /// Suggested split points for resumption.
    pub suggested_splits: Vec<String>,
    /// Summary of what was accomplished.
    pub completed_summary: String,
}

impl TokenExhaustionReport {
    /// Build a user-facing error message from this report.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::swarm::token_exhaustion::TokenExhaustionReport;
    ///
    /// let report = TokenExhaustionReport {
    ///     subtask_id: "st-002".into(),
    ///     tokens_used: 260_000,
    ///     tokens_limit: 256_000,
    ///     steps_completed: 15,
    ///     changed_files: vec![],
    ///     last_tool_calls: vec![],
    ///     errors_encountered: vec!["context too large".into()],
    ///     partial_output: "".into(),
    ///     recovery_hint: "Reduce scope".into(),
    /// };
    /// let msg = report.to_error_message();
    /// assert!(msg.contains("token"));
    /// ```
    pub fn to_error_message(&self) -> String {
        format!(
            "Sub-agent token exhaustion: subtask={} used={}/{} tokens, \
             steps={}, files_changed={}, recovery: {}",
            self.subtask_id,
            self.tokens_used,
            self.tokens_limit,
            self.steps_completed,
            self.changed_files.len(),
            self.recovery_hint,
        )
    }
}

impl RecoveryAdvice {
    /// Derive recovery advice from an exhaustion report.
    pub fn from_report(report: &TokenExhaustionReport) -> Self {
        let can_resume = report.steps_completed > 0;

        let mut suggested_splits = Vec::new();
        if !report.changed_files.is_empty() {
            suggested_splits.push(format!(
                "Resume editing: {}",
                report.changed_files.join(", ")
            ));
        }
        if report.steps_completed > 5 {
            suggested_splits.push(format!(
                "Continue from step {} with fresh context",
                report.steps_completed
            ));
        }
        if suggested_splits.is_empty() {
            suggested_splits.push("Retry with reduced scope".into());
        }

        let completed_summary = if report.steps_completed == 0 {
            "No steps completed before exhaustion.".into()
        } else {
            format!(
                "Completed {} steps, changed {} file(s).",
                report.steps_completed,
                report.changed_files.len()
            )
        };

        Self {
            can_resume,
            suggested_splits,
            completed_summary,
        }
    }
}

/// Extract forensic evidence (changed files, last tools, errors)
/// from a conversation history represented as tool call / result pairs.
///
/// `history` is a slice of `(tool_name, output, success)` tuples
/// representing the last N tool interactions.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::swarm::token_exhaustion::extract_evidence;
///
/// let history = vec![
///     ("edit", "wrote src/foo.rs", true),
///     ("bash", "cargo test", false),
/// ];
/// let (files, tools, errors) = extract_evidence(&history);
/// assert!(files.contains(&"src/foo.rs".to_string()));
/// assert!(tools.contains(&"edit".to_string()));
/// ```
pub fn extract_evidence(
    history: &[( &str, &str, bool )],
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut files = Vec::new();
    let mut tools = Vec::new();
    let mut errors = Vec::new();

    for (name, output, success) in history {
        tools.push((*name).to_string());
        if !success {
            errors.push(format!("{name}: {output}"));
        }
        // Naive file extraction: look for paths in edit/write tool output
        if *name == "edit" || *name == "write" {
            for word in output.split_whitespace() {
                if word.contains('/') || word.ends_with(".rs") || word.ends_with(".ts") {
                    files.push(word.trim_matches('"').to_string());
                }
            }
        }
    }

    files.sort();
    files.dedup();
    tools.sort();
    tools.dedup();

    (files, tools, errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_error_message_contains_key_fields() {
        let report = TokenExhaustionReport {
            subtask_id: "st-42".into(),
            tokens_used: 300_000,
            tokens_limit: 256_000,
            steps_completed: 30,
            changed_files: vec!["src/lib.rs".into()],
            last_tool_calls: vec!["edit".into()],
            errors_encountered: vec![],
            partial_output: "Half done".into(),
            recovery_hint: "Continue from item 4".into(),
        };
        let msg = report.to_error_message();
        assert!(msg.contains("st-42"));
        assert!(msg.contains("300000/256000"));
        assert!(msg.contains("Continue from item 4"));
    }

    #[test]
    fn recovery_advice_from_report_with_progress() {
        let report = TokenExhaustionReport {
            subtask_id: "st-1".into(),
            tokens_used: 256_000,
            tokens_limit: 256_000,
            steps_completed: 12,
            changed_files: vec!["src/main.rs".into(), "src/lib.rs".into()],
            last_tool_calls: vec!["edit".into()],
            errors_encountered: vec![],
            partial_output: "Partial".into(),
            recovery_hint: "Resume".into(),
        };
        let advice = RecoveryAdvice::from_report(&report);
        assert!(advice.can_resume);
        assert!(advice.suggested_splits.len() >= 2);
    }

    #[test]
    fn recovery_advice_from_report_no_progress() {
        let report = TokenExhaustionReport {
            subtask_id: "st-2".into(),
            tokens_used: 256_000,
            tokens_limit: 256_000,
            steps_completed: 0,
            changed_files: vec![],
            last_tool_calls: vec![],
            errors_encountered: vec![],
            partial_output: String::new(),
            recovery_hint: "Reduce scope".into(),
        };
        let advice = RecoveryAdvice::from_report(&report);
        assert!(!advice.can_resume);
        assert!(advice
            .suggested_splits
            .iter()
            .any(|s| s.contains("reduced scope")));
    }

    #[test]
    fn extract_evidence_finds_files_and_errors() {
        let history = vec![
            ("edit", "wrote src/foo.rs successfully", true),
            ("bash", "cargo test FAILED", false),
            ("edit", "wrote src/bar.rs", true),
        ];
        let (files, tools, errors) = extract_evidence(&history);
        assert!(files.contains(&"src/foo.rs".to_string()));
        assert!(files.contains(&"src/bar.rs".to_string()));
        assert!(tools.contains(&"edit".to_string()));
        assert!(tools.contains(&"bash".to_string()));
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("FAILED"));
    }

    #[test]
    fn extract_evidence_empty_history() {
        let history: Vec<(&str, &str, bool)> = vec![];
        let (files, tools, errors) = extract_evidence(&history);
        assert!(files.is_empty());
        assert!(tools.is_empty());
        assert!(errors.is_empty());
    }
}
