//! Result from a policy-gated quality command runner.

/// Success and combined output reported by a host command runner.
///
/// # Examples
/// ```
/// let output = codetether_forage_core::quality::QualityShellOutput { success: true, combined: String::new() };
/// assert!(output.success);
/// ```
pub struct QualityShellOutput {
    /// Whether the host reported successful command completion.
    pub success: bool,
    /// Combined output used for the quality summary.
    pub combined: String,
}
