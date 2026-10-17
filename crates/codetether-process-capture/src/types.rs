//! Data types for bounded bash output capture.

use std::process::ExitStatus;

/// Captured text and byte accounting for one output stream.
///
/// # Examples
/// ```rust
/// let stream = codetether_process_capture::CapturedStream::empty();
/// assert_eq!(stream.bytes, 0);
/// ```
pub struct CapturedStream {
    /// Retained lossy UTF-8 prefix.
    pub text: String,
    /// Total bytes drained, including discarded bytes.
    pub bytes: usize,
    /// Whether any bytes exceeded the retention limit.
    pub truncated: bool,
}

impl CapturedStream {
    /// Create an empty stream record.
    pub fn empty() -> Self {
        Self {
            text: String::new(),
            bytes: 0,
            truncated: false,
        }
    }
}

/// Exit status and both captured output streams.
///
/// Returned by [CaptureOutcome::Finished]. Use [Self::total_bytes] and
/// [Self::truncated] to inspect combined accounting.
///
/// # Examples
/// ```rust
/// use codetether_process_capture::{CapturedOutput, CapturedStream};
/// let make_output = |status| CapturedOutput {
///     status,
///     stdout: CapturedStream::empty(),
///     stderr: CapturedStream::empty(),
/// };
/// # let _: fn(std::process::ExitStatus) -> CapturedOutput = make_output;
/// ```
pub struct CapturedOutput {
    /// Exit status returned by the child.
    pub status: ExitStatus,
    /// Captured standard output.
    pub stdout: CapturedStream,
    /// Captured standard error.
    pub stderr: CapturedStream,
}

impl CapturedOutput {
    /// Total bytes drained from both streams.
    pub fn total_bytes(&self) -> usize {
        self.stdout.bytes + self.stderr.bytes
    }

    /// Whether either stream exceeded its retained byte limit.
    pub fn truncated(&self) -> bool {
        self.stdout.truncated || self.stderr.truncated
    }
}

/// Completion or timeout outcome of process capture.
///
/// # Examples
/// ```rust
/// let outcome = codetether_process_capture::CaptureOutcome::TimedOut;
/// assert!(matches!(outcome, codetether_process_capture::CaptureOutcome::TimedOut));
/// ```
pub enum CaptureOutcome {
    /// The child exited and its streams were drained.
    Finished(CapturedOutput),
    /// The deadline elapsed and process-tree cleanup was requested.
    TimedOut,
}
