//! Extract learning lines without changing their order.
/// Collect lines marked as learnings.
///
/// # Arguments
/// * `response` - Agent response text.
/// # Returns
/// Matching lines with surrounding whitespace removed.
/// # Examples
/// ```
/// assert_eq!(codetether_ralph_core::presentation::extract_learnings_static("learned X"), vec!["learned X"]);
/// ```

/// Extract learnings (static version)
pub fn extract_learnings_static(response: &str) -> Vec<String> {
    response
        .lines()
        .filter(|line| {
            line.contains("learned") || line.contains("Learning") || line.contains("# What")
        })
        .map(|line| line.trim().to_string())
        .collect()
}
/// Collect lines marked as learnings.
///
/// # Arguments
/// * `response` - Agent response text.
/// # Returns
/// Matching lines with surrounding whitespace removed.
/// # Examples
/// ```
/// assert_eq!(codetether_ralph_core::presentation::extract_learnings("learned X"), vec!["learned X"]);
/// ```

/// Extract learnings from response
pub fn extract_learnings(response: &str) -> Vec<String> {
    let mut learnings = Vec::new();

    for line in response.lines() {
        if line.contains("learned") || line.contains("Learning") || line.contains("# What") {
            learnings.push(line.trim().to_string());
        }
    }

    learnings
}
