//! Recognize successful empty codesearch output.

/// Return whether a successful codesearch result represents a no-match response.
///
/// # Examples
///
/// ```rust
/// use codetether_prompt_policy::is_codesearch_no_match_output;
///
/// assert!(is_codesearch_no_match_output(
///     "codesearch",
///     true,
///     "No matches found for pattern: example",
/// ));
/// assert!(!is_codesearch_no_match_output("read", true, "No matches found"));
/// ```
/// # Arguments
/// Receives a tool identifier, success flag and rendered output.
/// # Returns
/// True for a successful codesearch response containing the expected no-match text.
pub fn is_codesearch_no_match_output(tool_name: &str, success: bool, output: &str) -> bool {
    tool_name == "codesearch"
        && success
        && output
            .to_ascii_lowercase()
            .contains("no matches found for pattern:")
}
