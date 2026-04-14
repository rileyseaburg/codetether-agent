/// Return whether the tool requires interactive user input.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::runtime::is_interactive_tool;
///
/// assert!(is_interactive_tool("question"));
/// assert!(!is_interactive_tool("read"));
/// ```
pub fn is_interactive_tool(tool_name: &str) -> bool {
    matches!(tool_name, "question")
}

/// Return whether a provider name refers to a local CUDA-backed provider.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::runtime::is_local_cuda_provider;
///
/// assert!(is_local_cuda_provider("local_cuda"));
/// assert!(is_local_cuda_provider("local-cuda"));
/// assert!(!is_local_cuda_provider("openai"));
/// ```
pub fn is_local_cuda_provider(provider: &str) -> bool {
    matches!(provider, "local-cuda" | "local_cuda" | "localcuda")
}

/// Return the lightweight system prompt used for local CUDA providers.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::runtime::local_cuda_light_system_prompt;
///
/// let prompt = local_cuda_light_system_prompt();
/// assert!(prompt.contains("local CUDA assistant"));
/// ```
pub fn local_cuda_light_system_prompt() -> String {
    std::env::var("CODETETHER_LOCAL_CUDA_SYSTEM_PROMPT").unwrap_or_else(|_| {
        "You are CodeTether local CUDA assistant. Be concise and execution-focused. \
Use tools only when needed. For tool discovery, call list_tools first, then call specific tools with valid JSON arguments. \
Do not invent tool outputs."
            .to_string()
    })
}

/// Return whether a successful codesearch result represents a no-match response.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::runtime::is_codesearch_no_match_output;
///
/// assert!(is_codesearch_no_match_output(
///     "codesearch",
///     true,
///     "No matches found for pattern: example",
/// ));
/// assert!(!is_codesearch_no_match_output("read", true, "No matches found"));
/// ```
pub fn is_codesearch_no_match_output(tool_name: &str, success: bool, output: &str) -> bool {
    tool_name == "codesearch"
        && success
        && output
            .to_ascii_lowercase()
            .contains("no matches found for pattern:")
}
