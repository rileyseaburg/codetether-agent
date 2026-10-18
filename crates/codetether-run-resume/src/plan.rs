//! Resume plan data and CLI limit validation.
use anyhow::Result;
/// Prompt to resume a checkpoint and the remaining retry count.
/// # Examples
/// ```
/// let plan = codetether_run_resume::RunResumePlan { prompt: "continue".into(), remaining: 1 };
/// assert_eq!(plan.remaining, 1);
/// ```

pub struct RunResumePlan {
    /// Continuation prompt rendered from the checkpoint.
    pub prompt: String,
    /// Retries remaining after this resume attempt is selected.
    pub remaining: usize,
}
/// Reject a supplied zero retry limit.
/// # Arguments
/// * `limit` - Optional number of resume attempts.
/// # Returns
/// Unit for absent or positive limits.
/// # Errors
/// Returns the CLI validation error when the limit is zero.
/// # Examples
/// ```
/// assert!(codetether_run_resume::validate_auto_continue(None).is_ok());
/// assert!(codetether_run_resume::validate_auto_continue(Some(0)).is_err());
/// ```

pub fn validate_auto_continue(limit: Option<usize>) -> Result<()> {
    if matches!(limit, Some(0)) {
        anyhow::bail!("--auto-continue-until must be at least 1");
    }
    Ok(())
}
