//! Raw claimed-task execution result.
use anyhow::Result;
/// Status, output, error and session ID returned by successful task execution.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::outcome::TaskInner;
/// let result: TaskInner = Ok(("completed", Some("done".into()), None, None));
/// assert!(result.is_ok());
/// ```

pub type TaskInner = Result<(&'static str, Option<String>, Option<String>, Option<String>)>;
