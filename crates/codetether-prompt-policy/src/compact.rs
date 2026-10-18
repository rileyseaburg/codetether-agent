//! Compact local system prompt with an optional environment override.

/// Return the lightweight system prompt used for local CUDA providers.
///
/// # Examples
///
/// ```rust
/// use codetether_prompt_policy::local_cuda_light_system_prompt;
///
/// let prompt = local_cuda_light_system_prompt();
/// assert!(prompt.contains("local CUDA assistant"));
/// ```
/// # Arguments
/// Takes no arguments; reads the optional local prompt environment override.
/// # Returns
/// The configured prompt or built-in compact instructions.
pub fn local_cuda_light_system_prompt() -> String {
    std::env::var("CODETETHER_LOCAL_CUDA_SYSTEM_PROMPT").unwrap_or_else(|_| {
        "You are CodeTether local CUDA assistant. Be concise and execution-focused. \
Use tools only when needed. For tool discovery, call list_tools first, then call specific tools with valid JSON arguments. \
Do not invent tool outputs."
            .to_string()
    })
}
