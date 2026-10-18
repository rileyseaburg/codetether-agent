//! Established execution language markers for build-mode decisions.

/// Apply the established looks like build execution request rule.
/// # Arguments
/// Receives the message, tool or workspace inputs named in the signature.
/// # Returns
/// The decision or matching paths without executing tools.
/// # Examples
/// ```
/// assert!(codetether_build_policy::looks_like_build_execution_request("fix the parser"));
/// ```
pub fn looks_like_build_execution_request(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let keywords = [
        "fix",
        "patch",
        "implement",
        "add",
        "update",
        "edit",
        "change",
        "refactor",
        "debug",
        "investigate",
        "run",
        "test",
        "build",
        "compile",
        "create",
        "remove",
        "rename",
        "wire",
        "hook up",
    ];
    keywords.iter().any(|k| lower.contains(k))
}
